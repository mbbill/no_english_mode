#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows::core::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::{
    Foundation::*,
    UI::{
        Accessibility::*, Input::Ime::*, Input::KeyboardAndMouse::*, Shell::*,
        WindowsAndMessaging::*,
    },
};

unsafe extern "system" fn event_hook_callback(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    // Get the thread ID of the window that has come to the foreground.
    let thread_id = GetWindowThreadProcessId(hwnd, None);

    // Get the current keyboard layout
    let hkl = GetKeyboardLayout(thread_id);

    // Check if the current IME status matches Chinese (0x804)
    if ((hkl.0 as u32 & 0xffff) == 0x804) && (hwnd.0 != 0) {
        // Get the ime window handle
        let ime_hwnd = ImmGetDefaultIMEWnd(hwnd);
        // Switch the IME state
        println!("Chinese input method detected, forcing Chinese mode.");
        // Sometimes the message will miss if we don't sleep for a little while.
        thread::sleep(Duration::from_millis(10));
        SendMessageW(
            ime_hwnd,
            WM_IME_CONTROL,
            WPARAM(IMC_SETCONVERSIONMODE as usize),
            LPARAM(1025), // Chinese
        );
    }
}

// Make sure this is the same as the icon id in the rc file
const IDI_ICON1: u16 = 101;
const IDM_EXIT: u32 = 1001;
const NOTIFYICONMESSAGE: u32 = WM_USER + 100;

fn add_tray_icon(hwnd: HWND) -> windows::core::Result<()> {
    let h_instance = unsafe { GetModuleHandleW(None) }?;

    let mut nid = NOTIFYICONDATAW {
        uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
        hWnd: hwnd,
        uCallbackMessage: NOTIFYICONMESSAGE,
        hIcon: unsafe { LoadIconW(h_instance, PCWSTR(IDI_ICON1 as *const u16))? }, // Load the app icon
        szTip: [0; 128],               // Tooltip text
        ..Default::default()
    };

    unsafe {
        // Convert the tooltip to a wide string
        let tooltip = "NoEnglishMode\0";
        for (i, c) in tooltip.encode_utf16().enumerate() {
            nid.szTip[i] = c;
        }

        // Add the icon
        Shell_NotifyIconW(NIM_ADD, &mut nid);
    }
    Ok(())
}


unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {

    match msg {
        NOTIFYICONMESSAGE => match lparam.0 as u32 {
            WM_RBUTTONUP => {
                let mut point = POINT::default();
                let _ = GetCursorPos(&mut point);

                let hmenu = CreatePopupMenu().unwrap();
                let menu_item_str: &'static str = "Exit";
                let menu_item_wstr: Vec<u16> = menu_item_str
                    .encode_utf16()
                    .chain(std::iter::once(0))
                    .collect();
                let _ = AppendMenuW(hmenu, MENU_ITEM_FLAGS(0), IDM_EXIT as usize, PCWSTR(menu_item_wstr.as_ptr()));

                // Set the foreground window to the current window to ensure the menu closes properly
                SetForegroundWindow(hwnd);

                // Track the popup menu at the cursor position
                TrackPopupMenu(hmenu, TPM_RIGHTBUTTON, point.x, point.y, 0, hwnd, None);

                // Required to make sure the menu closes on time
                let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
            }
            _ => {}
        },
        WM_COMMAND => {
            match wparam.0 as u32 {
                IDM_EXIT => {
                    // Handle the "Exit" menu item
                    PostQuitMessage(0);
                }
                _ => {}
            }
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }

    LRESULT(0)
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let class_name_str: &'static str = "hidden_window_class";
        let class_name_wide: Vec<u16> = class_name_str
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let class_name = PCWSTR(class_name_wide.as_ptr());
        let window_name_str: &'static str = "hidden_window";
        let window_name_str: Vec<u16> = window_name_str
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let window_name = PCWSTR(window_name_str.as_ptr());

        let instance = GetModuleHandleW(None)?;

        // Define a window class
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: HINSTANCE(instance.0),
            lpszClassName: class_name,
            ..Default::default()
        };

        // Register the window class
        let class_atom = RegisterClassW(&wc);
        if class_atom == 0 {
            // Handle error
            return Err(windows::core::Error::from_win32());
        }

        // Create the hidden window
        let hwnd = CreateWindowExW(
            Default::default(),
            class_name,
            window_name,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        if hwnd.0 == 0 {
            // Handle error
            return Err(windows::core::Error::from_win32());
        }

        // Add the tray icon using 'hwnd'
        add_tray_icon(hwnd)?;

        // Set the hook
        let hook = SetWinEventHook(
            EVENT_OBJECT_FOCUS,
            EVENT_OBJECT_FOCUS,
            None, // Handle to the DLL with the callback function, None for the current process
            Some(event_hook_callback),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        // Check if the hook was set successfully
        if hook.0 == 0 {
            // Handle the error if the hook is not set
            println!("Failed to set hook!");
            return Err(Error::from_win32());
        }

        // Message loop
        let mut message = MSG::default();
        while GetMessageW(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        // Unhook before exit
        UnhookWinEvent(hook);
    }

    Ok(())
}

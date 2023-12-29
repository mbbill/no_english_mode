# NO English Mode!
防止微软拼音输入法启动切换到英文模式

和AutoHotkey脚本的区别是这个程序监听focus事件而不是一直循环，运行在后台几乎不消耗资源，用户完全无感知。

![Screenshot](assets/screenshot.png)

# Install
Download the latest release and run the executable. Run `copy_to_startup.bat` if needed.
There's a tray icon to let you know it's running in the background.

# Build
`cargo build [--release]`

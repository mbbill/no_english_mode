@echo off
setlocal

:: Get the path to the current user's Startup folder
set STARTUP_FOLDER=%USERPROFILE%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup

:: Specify the path to your application's executable
set APP_EXE=no_english_mode.exe

:: Copy the application to the Startup folder
copy /Y "%APP_EXE%" "%STARTUP_FOLDER%"

:: Inform the user
echo Your application has been copied to the Startup folder.
pause
endlocal

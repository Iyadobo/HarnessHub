@echo off
cd /d "%~dp0"
if exist "%~dp0.local\bin\harnesshub.cmd" goto installedcmd
if exist "%~dp0.local\bin\harnesshub.exe" goto installed
if not exist "%~dp0target\release\harnesshub.exe" goto build
"%~dp0target\release\harnesshub.exe" %*
exit /b %errorlevel%
:build
cargo run -p harnesshub --release --locked -- %*
exit /b %errorlevel%
:installed
"%~dp0.local\bin\harnesshub.exe" %*
exit /b %errorlevel%
:installedcmd
call "%~dp0.local\bin\harnesshub.cmd" %*
exit /b %errorlevel%

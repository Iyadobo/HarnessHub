@echo off
cd /d "%~dp0"
if not exist "%~dp0target\release\harnesshub.exe" goto build
"%~dp0target\release\harnesshub.exe" %*
exit /b %errorlevel%
:build
cargo run -p harnesshub --release --locked -- %*

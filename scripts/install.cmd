@echo off
setlocal EnableExtensions

set "INSTALLER_URL=https://github.com/weirdo-adam/redmine-mcp-server/releases/latest/download/install.ps1"

where powershell.exe >nul 2>nul
if %ERRORLEVEL% EQU 0 goto use_powershell

where pwsh.exe >nul 2>nul
if %ERRORLEVEL% EQU 0 goto use_pwsh

echo PowerShell is required to install Redmine MCP server.
exit /b 1

:use_powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "irm '%INSTALLER_URL%' | iex"
exit /b %ERRORLEVEL%

:use_pwsh
pwsh.exe -NoProfile -ExecutionPolicy Bypass -Command "irm '%INSTALLER_URL%' | iex"
exit /b %ERRORLEVEL%

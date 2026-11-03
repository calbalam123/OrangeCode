@echo off
chcp 65001 >nul
if "%~1"=="" (
  echo Orange Code - 실행할 .orange 파일을 이 실행기로 열어 주세요.
  pause
  exit /b 1
)
"%~dp0..\native\target\release\orange.exe" "%~1"
echo.
pause

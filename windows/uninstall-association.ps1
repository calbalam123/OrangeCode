$ErrorActionPreference = 'Stop'
Remove-Item -Path 'HKCU:\Software\Classes\.orange' -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path 'HKCU:\Software\Classes\OrangeCode.Source' -Recurse -Force -ErrorAction SilentlyContinue
Write-Host '.orange 파일 연결을 해제했습니다.' -ForegroundColor Green
Read-Host '엔터를 누르면 종료합니다'
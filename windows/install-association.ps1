$ErrorActionPreference = 'Stop'
$runtime = Join-Path $PSScriptRoot 'orange.exe'
$launcher = Join-Path $PSScriptRoot 'orange-launcher.bat'
if (-not (Test-Path $runtime)) { throw "orange.exe를 찾을 수 없습니다: $runtime`norange.exe와 이 스크립트를 같은 폴더에 두세요." }
if (-not (Test-Path $launcher)) { throw "orange-launcher.bat를 찾을 수 없습니다: $launcher" }
$extensionKey = 'HKCU:\Software\Classes\.orange'
$programKey = 'HKCU:\Software\Classes\OrangeCode.Source\shell\open\command'
New-Item -Path $extensionKey -Force | Out-Null
Set-Item -Path $extensionKey -Value 'OrangeCode.Source'
New-Item -Path $programKey -Force | Out-Null
$command = 'cmd.exe /k ""{0}" "%1""' -f $launcher
Set-Item -Path $programKey -Value $command
Write-Host '.orange 파일 연결이 완료되었습니다.' -ForegroundColor Green
Write-Host '이제 .orange 파일을 더블클릭하면 Orange Code가 실행됩니다.'
Write-Host '연결이 바로 적용되지 않으면 파일 탐색기를 다시 시작하거나 로그아웃 후 로그인하세요.'
Read-Host '엔터를 누르면 종료합니다'
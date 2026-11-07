# Mac Ventura 0.2 — Arch Linux Edition

맥 벤츄라는 Arch Linux + KDE Plasma 기반의 한국어 중심 데스크톱 Linux 프로젝트입니다. macOS Ventura에서 영감을 받은 시각적 방향을 사용하지만 Apple macOS나 독점 코드는 포함하지 않습니다.

## 포함 기능

- KDE Plasma 데스크톱 및 SDDM 그래픽 로그인
- Papirus 아이콘, Kvantum 테마 엔진, Breeze 기반 테마 구성 요소
- Dolphin 파일 관리자, Konsole 터미널, Kate 편집기
- Firefox, Thunderbird, VLC, LibreOffice, GIMP
- Discover 앱 센터, PackageKit, Flatpak 지원
- 한국어 UTF-8 로케일, Noto CJK 글꼴, Fcitx5 Hangul 입력기
- NetworkManager, Wi-Fi 도구, Bluetooth, KDE Connect
- 프린터 서비스, USB/디스크 관리, 전원 관리
- 스크린샷, 문서 뷰어, 계산기, 시스템 모니터, 디스크 파티션 도구
- UEFI/BIOS 부팅 설정은 공식 Archiso releng 프로필을 기반으로 유지

## 프로젝트 상태

0.2는 **라이브 데스크톱 ISO 프로토타입**입니다. ISO를 자동 빌드하는 GitHub Actions 워크플로를 포함합니다. 설치 프로그램, 자체 업데이트 저장소, 서명된 패키지 저장소, macOS 수준의 모든 독점 서비스는 아직 구현되어 있지 않습니다.

## 자동 ISO 빌드

GitHub 저장소에서 **Actions → Mac Ventura ISO Build → Run workflow**를 선택하거나, `MacVentura/` 파일을 변경해 main에 반영하면 빌드가 시작됩니다. 성공하면 Actions 실행 화면의 Artifacts에서 `macventura-iso`를 내려받을 수 있습니다.

## 로컬 빌드 (Arch Linux)

```bash
chmod +x ./*.sh
./prepare-build-env.sh
./check-project.sh
./build-iso.sh
```

ISO 출력 위치: `out/`

## Windows + VS Code

Windows에서 VS Code와 Remote - SSH 확장을 사용해 Arch Linux 가상머신에 접속한 뒤, 프로젝트를 VM의 Linux 파일시스템에 두고 빌드하는 것을 권장합니다. ISO 빌드는 네트워크와 충분한 메모리/디스크 공간이 필요합니다.

## 테스트 전 주의

- 첫 부팅은 반드시 VirtualBox, VMware 또는 QEMU에서 테스트하세요.
- 실제 하드웨어에 설치하기 전에 네트워크, 그래픽, 한국어 입력, 오디오, Bluetooth, 절전/복귀를 검증하세요.
- 라이브 ISO는 설치된 시스템과 다릅니다. 이 버전에는 그래픽 설치 프로그램이 포함되어 있지 않습니다.
- Arch Linux는 rolling release이므로 패키지 버전과 빌드 결과는 빌드 날짜에 따라 달라집니다.

## 라이선스

자체 스크립트와 설정은 MIT License입니다. 포함 패키지와 Arch Linux 구성 요소는 각자의 라이선스를 따릅니다. Apple 및 macOS는 Apple Inc.의 상표이며 본 프로젝트는 비공식입니다.

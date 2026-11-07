# Mac Ventura 0.1 — Arch Linux Edition

**맥 벤츄라**는 Arch Linux와 KDE Plasma를 기반으로 제작하는 한국어 중심의 개인용 Linux 배포판 프로젝트입니다.
macOS Ventura에서 영감을 받은 데스크톱 경험을 목표로 하지만, Apple macOS 자체나 Apple의 독점 코드·시스템 파일을 포함하지 않습니다.

## 확정 사양

- 배포판 기반: Arch Linux (x86_64)
- ISO 제작: 공식 `archiso` / `mkarchiso`
- 데스크톱: KDE Plasma 6 계열 (Arch 저장소의 현재 패키지 버전)
- 디스플레이 관리자: SDDM
- 기본 언어: 한국어 (`ko_KR.UTF-8`)
- 한국어 입력: Fcitx5 + Hangul
- 부팅 목표: UEFI 및 BIOS 부팅 (공식 releng 프로필의 부트 설정 유지)
- 개발: Windows + VS Code, 빌드: Arch Linux 가상머신

## 프로젝트 상태

이 저장소는 **첫 번째 커스터마이징 빌드 킷**입니다. ISO 빌드 스크립트와 데스크톱 기본 구성을 포함합니다.
실제 ISO 파일은 Arch Linux 환경에서 빌드해야 합니다. 이 프로젝트 파일 자체가 이미 빌드된 ISO는 아닙니다.

## 권장 빌드 PC / 가상머신

- CPU: 4코어 이상 권장
- RAM: 8GB 이상 권장 (가능하면 12GB)
- 디스크: 여유 공간 40~60GB 이상
- VM: VirtualBox, VMware 또는 QEMU/KVM
- VM 펌웨어: UEFI 권장, BIOS 부팅도 테스트

## 빠른 시작

Arch Linux VM에서 터미널을 열고 프로젝트 디렉터리로 이동합니다.

```bash
chmod +x ./*.sh
./prepare-build-env.sh
./check-project.sh
./build-iso.sh
```

완료되면 `out/` 폴더에 `macventura-0.1-x86_64.iso` 형태의 ISO가 생성됩니다.
파일명은 빌드 스크립트와 archiso 프로필의 버전 설정에 따릅니다.

## Windows + VS Code 작업

1. Windows에 VS Code를 설치합니다.
2. Arch Linux VM을 설치하고 네트워크를 연결합니다.
3. VS Code의 Remote - SSH 확장을 설치합니다.
4. VM에서 SSH 서버를 켜고 VS Code로 접속합니다.
5. 프로젝트 폴더를 VM의 Linux 파일시스템(예: `~/MacVentura`)에 둡니다.
6. VM 터미널에서 위 빌드 명령을 실행합니다.

Windows 파일시스템(`/mnt/c`)보다 VM의 Linux 파일시스템 안에서 빌드하는 것을 권장합니다.

## 주의

- `build-iso.sh`는 공식 `/usr/share/archiso/configs/releng` 프로필을 복사해 사용합니다.
- Archiso의 기본 부트로더 설정을 유지하고, 이 프로젝트의 패키지와 rootfs 오버레이를 추가합니다.
- Arch Linux 패키지 저장소는 rolling release이므로 같은 프로젝트라도 빌드 날짜에 따라 포함 패키지 버전이 달라질 수 있습니다.
- 첫 빌드는 네트워크 연결이 필요하며 다운로드 용량이 클 수 있습니다.
- 실제 PC에 설치하기 전 반드시 VM에서 부팅, 네트워크, 한국어 입력, 디스플레이를 확인하세요.
- 이 ISO는 현재 라이브 환경 프로토타입입니다. 설치 프로그램은 아직 포함하지 않습니다.

## 라이선스 / 상표

프로젝트의 자체 스크립트와 설정 파일은 MIT License로 제공합니다.
Arch Linux, KDE Plasma 및 각 패키지는 각각의 라이선스를 따릅니다.
Apple 및 macOS는 Apple Inc.의 상표이며, 본 프로젝트는 비공식 개인 프로젝트입니다.

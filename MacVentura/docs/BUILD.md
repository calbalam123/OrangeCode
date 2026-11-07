# 빌드 및 테스트 가이드

## A. Arch Linux 가상머신 준비

VirtualBox/VMware에 최신 Arch Linux x86_64를 설치합니다.
VM 권장값은 CPU 4코어, RAM 8~12GB, 디스크 50GB 이상입니다.
네트워크를 켜고 Arch Linux를 최신 상태로 업데이트합니다.

## B. VS Code 연결

VM에서 SSH를 사용할 경우:

```bash
sudo pacman -S --needed openssh
sudo systemctl enable --now sshd
ip addr
```

Windows VS Code에서 Remote - SSH 확장으로 VM에 접속합니다.
프로젝트 폴더는 VM 내부 `~/MacVentura`에 둡니다.

## C. 빌드

```bash
cd ~/MacVentura
chmod +x ./*.sh
./prepare-build-env.sh
./check-project.sh
./build-iso.sh
```

빌드가 끝나면 `out/` 아래 ISO가 생성됩니다.

## D. QEMU에서 부팅 확인

UEFI 부팅 테스트 예시:

```bash
qemu-system-x86_64 \
  -enable-kvm -m 4096 -smp 4 \
  -bios /usr/share/edk2/x64/OVMF_CODE.4m.fd \
  -cdrom out/macventura-0.1-x86_64.iso
```

VM에서 KVM/OVMF 경로가 다르면 해당 환경의 파일 경로에 맞게 조정합니다.
KVM을 사용할 수 없는 환경에서는 `-enable-kvm`을 빼고 실행할 수 있습니다.

## E. 확인 항목

- 라이브 ISO가 부팅되는가?
- SDDM에서 `liveuser`로 자동 로그인되는가?
- KDE Plasma 세션이 시작되는가?
- 네트워크가 연결되는가?
- 시스템 언어가 한국어인가?
- Fcitx5에서 한글 입력이 되는가?
- 파일 관리자와 터미널이 실행되는가?

## F. 알려진 제한

이 단계는 라이브 데스크톱 ISO 프로토타입입니다.
하드디스크 설치 프로그램, 자동 파티셔닝, 업데이트 서버, 자체 패키지 저장소, 서명 체계는 다음 단계에서 개발해야 합니다.

#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -eq 0 ]]; then
  echo "일반 사용자로 실행하세요. 필요한 명령에 sudo를 사용합니다."
  exit 1
fi

if ! grep -qi 'arch' /etc/os-release; then
  echo "이 스크립트는 Arch Linux 빌드 환경을 대상으로 합니다."
  echo "Ubuntu/Windows에서는 Arch Linux 가상머신을 준비하세요."
  exit 1
fi

sudo pacman -Syu --needed --noconfirm archiso arch-install-scripts \
  qemu-desktop edk2-ovmf git rsync squashfs-tools dosfstools \
  mtools libisoburn

echo
echo "빌드 도구 설치 완료."
echo "다음: ./check-project.sh && ./build-iso.sh"

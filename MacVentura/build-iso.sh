#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROFILE_SRC="/usr/share/archiso/configs/releng"
BUILD_ROOT="$ROOT/.build"
PROFILE="$BUILD_ROOT/profile"
WORK="$BUILD_ROOT/work"
OUT="$ROOT/out"

if [[ "${EUID}" -ne 0 ]]; then
  echo "ISO 빌드는 root 권한이 필요합니다."
  echo "다시 실행합니다: sudo $0"
  exec sudo -- "$0" "$@"
fi

if ! grep -qi 'arch' /etc/os-release; then
  echo "오류: Arch Linux 환경에서만 실행하도록 구성했습니다."
  exit 1
fi

if ! command -v mkarchiso >/dev/null 2>&1; then
  echo "mkarchiso가 없습니다. 먼저 ./prepare-build-env.sh 를 실행하세요."
  exit 1
fi

if [[ ! -d "$PROFILE_SRC" ]]; then
  echo "공식 releng 프로필을 찾을 수 없습니다: $PROFILE_SRC"
  echo "archiso 패키지 설치를 확인하세요."
  exit 1
fi

echo "== Mac Ventura ISO 빌드 준비 =="
rm -rf -- "$PROFILE" "$WORK"
mkdir -p "$PROFILE" "$WORK" "$OUT"

# 공식 releng의 BIOS/UEFI 부트 설정과 필수 파일을 유지합니다.
cp -a "$PROFILE_SRC/." "$PROFILE/"

# 프로젝트 패키지를 공식 releng 패키지 목록에 추가합니다.
cat "$ROOT/profile/packages.x86_64" >> "$PROFILE/packages.x86_64"

# 프로젝트 rootfs 오버레이를 적용합니다.
cp -a "$ROOT/profile/airootfs/." "$PROFILE/"

# 라이브 데스크톱과 네트워크 서비스를 부팅 시 활성화합니다.
mkdir -p "$PROFILE/airootfs/etc/systemd/system/graphical.target.wants"
mkdir -p "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants"
ln -sfn /usr/lib/systemd/system/sddm.service \
  "$PROFILE/airootfs/etc/systemd/system/graphical.target.wants/sddm.service"
ln -sfn /usr/lib/systemd/system/NetworkManager.service \
  "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants/NetworkManager.service"

# sudoers 파일은 반드시 0440 권한으로 설치합니다.
if grep -q '^file_permissions=(' "$PROFILE/profiledef.sh"; then
  sed -i '/^file_permissions=(/a\\  ["/etc/sudoers.d/10-liveuser"]="0:0:0440"' \
    "$PROFILE/profiledef.sh"
else
  cat >> "$PROFILE/profiledef.sh" <<'EOF_PERMS'

file_permissions=(
  ["/etc/sudoers.d/10-liveuser"]="0:0:0440"
)
EOF_PERMS
fi

# 한국어 UTF-8 로케일을 이미지에서 생성하도록 설정합니다.
if [[ -f "$PROFILE/airootfs/etc/locale.gen" ]]; then
  sed -i 's/^#ko_KR.UTF-8 UTF-8/ko_KR.UTF-8 UTF-8/' \
    "$PROFILE/airootfs/etc/locale.gen"
fi

# 공식 프로필 이름/라벨 등만 안전하게 변경합니다.
python - "$PROFILE/profiledef.sh" <<'PY'
from pathlib import Path
import re, sys
p = Path(sys.argv[1])
s = p.read_text(encoding="utf-8")
values = {
    "iso_name": "macventura",
    "iso_label": "MACVENTURA_01",
    "iso_publisher": "Mac Ventura Project",
    "iso_application": "Mac Ventura Live Desktop",
    "iso_version": "0.1",
}
for key, value in values.items():
    pattern = rf"(?m)^({key}=).*?$"
    replacement = lambda m, v=value: f'{m.group(1)}="{v}"'
    s, count = re.subn(pattern, replacement, s, count=1)
    if count == 0:
        s += f'\n{key}="{value}"\n'
p.write_text(s, encoding="utf-8")
PY

echo "== mkarchiso 실행 =="
mkarchiso -v -w "$WORK" -o "$OUT" "$PROFILE"

echo
echo "빌드 완료. 생성된 파일:"
find "$OUT" -maxdepth 1 -type f -name '*.iso' -printf '%p (%s bytes)\n'

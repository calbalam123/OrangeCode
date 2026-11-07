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
  exec sudo -- "$0" "$@"
fi

if ! grep -qi 'arch' /etc/os-release; then
  echo "오류: Arch Linux 환경에서만 실행할 수 있습니다."
  exit 1
fi
command -v mkarchiso >/dev/null 2>&1 || {
  echo "mkarchiso가 없습니다. archiso 패키지를 설치하세요."
  exit 1
}
[[ -d "$PROFILE_SRC" ]] || {
  echo "공식 releng 프로필을 찾을 수 없습니다: $PROFILE_SRC"
  exit 1
}

echo "== Mac Ventura ISO 빌드 준비 =="
rm -rf -- "$PROFILE" "$WORK"
mkdir -p "$PROFILE" "$WORK" "$OUT"
cp -a "$PROFILE_SRC/." "$PROFILE/"

# 공식 부트 설정은 유지하고 Mac Ventura 패키지와 rootfs 오버레이를 추가합니다.
cat "$ROOT/profile/packages.x86_64" >> "$PROFILE/packages.x86_64"
mkdir -p "$PROFILE/airootfs"\ncp -a "$ROOT/profile/airootfs/." "$PROFILE/airootfs/"

# 라이브 세션의 시스템 계정 및 sudo 권한을 보장합니다.
install -d -m 0755 "$PROFILE/airootfs/etc/sudoers.d"
chmod 0440 "$PROFILE/airootfs/etc/sudoers.d/10-liveuser"

# systemd 서비스 활성화 및 그래픽 로그인 기본 타깃 설정.
install -d "$PROFILE/airootfs/etc/systemd/system/graphical.target.wants"
install -d "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants"
ln -sfn /usr/lib/systemd/system/sddm.service \
  "$PROFILE/airootfs/etc/systemd/system/graphical.target.wants/sddm.service"
ln -sfn /usr/lib/systemd/system/NetworkManager.service \
  "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants/NetworkManager.service"
ln -sfn /usr/lib/systemd/system/bluetooth.service \
  "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants/bluetooth.service"
ln -sfn /usr/lib/systemd/system/cups.service \
  "$PROFILE/airootfs/etc/systemd/system/multi-user.target.wants/cups.service"
ln -sfn /usr/lib/systemd/system/graphical.target \
  "$PROFILE/airootfs/etc/systemd/system/default.target"

# 한국어 로케일 활성화.
install -d "$PROFILE/airootfs/etc"
if [[ ! -f "$PROFILE/airootfs/etc/locale.gen" ]]; then
  : > "$PROFILE/airootfs/etc/locale.gen"
fi
if ! grep -q '^ko_KR.UTF-8 UTF-8' "$PROFILE/airootfs/etc/locale.gen"; then
  printf '\nko_KR.UTF-8 UTF-8\n' >> "$PROFILE/airootfs/etc/locale.gen"
fi

# ISO 이름과 버전 설정.
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
    "iso_version": "0.2",
}
for key, value in values.items():
    pattern = rf"(?m)^({key}=).*?$"
    s, count = re.subn(pattern, lambda m, v=value: f'{m.group(1)}="{v}"', s, count=1)
    if count == 0:
        s += f'\n{key}="{value}"\n'
p.write_text(s, encoding="utf-8")
PY

echo "== mkarchiso 실행 =="
mkarchiso -v -w "$WORK" -o "$OUT" "$PROFILE"

echo
echo "빌드 완료:"
find "$OUT" -maxdepth 1 -type f -name '*.iso' -printf '%p (%s bytes)\n'

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
required=(
  "README.md"
  "build-iso.sh"
  "profile/packages.x86_64"
  "profile/airootfs/etc/locale.conf"
  "profile/airootfs/etc/sysusers.d/macventura.conf"
  "profile/airootfs/etc/sddm.conf.d/10-macventura.conf"
)

failed=0
for item in "${required[@]}"; do
  if [[ -f "$ROOT/$item" ]]; then
    printf '[OK] %s\n' "$item"
  else
    printf '[누락] %s\n' "$item"
    failed=1
  fi
done

for script in "$ROOT"/*.sh; do
  bash -n "$script" || failed=1
done

if [[ "$failed" -ne 0 ]]; then
  echo "프로젝트 검사 실패"
  exit 1
fi

echo "프로젝트 기본 검사 통과"

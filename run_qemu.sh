#!/usr/bin/env bash
set -euo pipefail

APP="target/x86_64-unknown-uefi/release/UefiEditor.efi"
OVMF="/usr/share/OVMF/OVMF_CODE_4M.fd"

test -f "$APP"        # ensure built
rm -rf esp && mkdir -p esp/EFI/BOOT
cp "$APP" esp/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 \
  -drive if=pflash,format=raw,unit=0,readonly=on,file=./OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,unit=1,file=./OVMF_VARS_4M_work.fd \
  -drive file=fat:rw:esp,format=raw
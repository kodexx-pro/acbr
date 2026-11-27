#!/usr/bin/env bash
set -euo pipefail
root_dir=$(cd "$(dirname "$0")/.." && pwd)
mirror_dir=${ACBR_MIRROR_DIR:-/home/ubuntu/acbr-mirror-data/trunk2}
lazarus_dir=${LAZARUS_DIR:-/usr/lib/lazarus/3.0}
revision=9f42f9a3375cd11e369be1ecdcc570b7dcaff814
work_dir=$(mktemp -d "${TMPDIR:-/tmp}/acbr-pascal.XXXXXXXX")
trap 'rm -rf -- "$work_dir"' EXIT
for command in git fpc cargo python3 tar; do command -v "$command" >/dev/null || { printf 'Comando ausente: %s\n' "$command" >&2; exit 1; }; done
test -d "$lazarus_dir/components/lazutils" || { printf 'Defina LAZARUS_DIR para um Lazarus com os fontes instalados.\n' >&2; exit 1; }
git -C "$mirror_dir" archive "$revision" Fontes/ACBrComum Fontes/ACBrDiversos/ACBrValidador.pas Fontes/Terceiros/synalist Fontes/Terceiros/FastStringReplace Fontes/Terceiros/GZIPUtils | tar -x -C "$work_dir"
mkdir -p "$work_dir/build"
fpc_args=(-v0 -dNOGUI -Mdelphi "-Fu$work_dir/Fontes/ACBrComum" "-Fu$work_dir/Fontes/ACBrDiversos" "-Fu$work_dir/Fontes/Terceiros/synalist" "-Fu$work_dir/Fontes/Terceiros/FastStringReplace" "-Fu$work_dir/Fontes/Terceiros/GZIPUtils" "-Fi$work_dir/Fontes/ACBrComum" "-Fu$lazarus_dir/components/lazutils" "-Fu$lazarus_dir/lcl" "-FU$work_dir/build" "-FE$work_dir/build")
for source in original_validador original_format original_calc original_details; do
 fpc "${fpc_args[@]}" "$root_dir/tests/pascal/$source.pas"
done
cd "$root_dir"
export ACBR_MIRROR_DIR="$mirror_dir"
ACBR_FULL_ORACLE="$work_dir/build/original_validador" python3 scripts/verify_validador_differential.py
ACBR_FORMAT_ORACLE="$work_dir/build/original_format" python3 scripts/verify_format_differential.py
ACBR_CALC_ORACLE="$work_dir/build/original_calc" python3 scripts/verify_calc_differential.py

ACBR_DETAILS_ORACLE="$work_dir/build/original_details" python3 scripts/verify_details_differential.py

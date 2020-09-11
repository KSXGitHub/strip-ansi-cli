#! /bin/bash
set -o errexit -o pipefail

mkdir ./flatten

[ -d ./downloads ] || {
	echo Folder ./downloads does not exist >/dev/stderr
	exit 1
}

# shellcheck disable=SC2012
ls ./downloads | while read -r name; do
	case "$name" in
	*wasm*) suffix=.wasm ;;
	*windows*) suffix=.exe ;;
	*) suffix='' ;;
	esac

	src="./downloads/${name}/strip-ansi${suffix}"
	dst="./flatten/${name}${suffix}"
	cp -v "$src" "$dst"
done

cp -v ./exports/* ./flatten/

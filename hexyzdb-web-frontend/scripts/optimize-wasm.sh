#!/usr/bin/env bash

set -e

if [ "$TRUNK_PROFILE" == "debug" ]; then
	WASM_OPT_ARGS="-g"
elif [ "$TRUNK_PROFILE" == "release" ]; then
	WASM_OPT_ARGS="-Os"
else
	printf "Unknown \$TRUNK_PROFILE: \"$TRUNK_PROFILE\"\n"
	exit 1
fi

find "$TRUNK_STAGING_DIR" \
	-iname '*.wasm' \
	-exec wasm-opt $WASM_OPT_ARGS {} -o {} \;

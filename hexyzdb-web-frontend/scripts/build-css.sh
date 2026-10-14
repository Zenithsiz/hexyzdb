#!/usr/bin/env bash

set -e

if [ "$TRUNK_PROFILE" == "debug" ]; then
	ESBUILD_ARGS=""
elif [ "$TRUNK_PROFILE" == "release" ]; then
	ESBUILD_ARGS="--minify"
else
	printf "Unknown \$TRUNK_PROFILE: \"$TRUNK_PROFILE\"\n"
	exit 1
fi

esbuild \
	$ESBUILD_ARGS \
	$(find "$TRUNK_SOURCE_DIR/css/" -iname '*.css') \
	--outdir="$TRUNK_STAGING_DIR/css/"

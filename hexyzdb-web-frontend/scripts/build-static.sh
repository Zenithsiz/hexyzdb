#!/usr/bin/env bash

set -e

if [ "$TRUNK_PROFILE" == "debug" ]; then
	CARGO_PROFILE="debug"
elif [ "$TRUNK_PROFILE" == "release" ]; then
	CARGO_PROFILE="release"
else
	printf "Unknown \$TRUNK_PROFILE: \"$TRUNK_PROFILE\"\n"
	exit 1
fi

cargo run \
	--manifest-path="../Cargo.toml" \
	--profile="$CARGO_PROFILE" \
	--bin hexyzdb-web-frontend-static \
	-- \
	--output-dir="$TRUNK_STAGING_DIR/static/"

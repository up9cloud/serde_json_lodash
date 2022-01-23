#!/bin/bash -e

./lint.sh

# major, minor, patch
if [ -z "$1" ]; then
	_type=patch
fi

# Similar as npm version, but without `v` prefix, e.q. npm: `v1.0.0` vs bump `1.0.0`
cargo bump $_type --git-tag
git push
git push --tags

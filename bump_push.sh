#!/bin/bash -xe

# Similar as npm version, but without `v` prefix, e.q. npm: `v1.0.0` vs bump `1.0.0`
cargo bump patch --git-tag
git push
git push --tags

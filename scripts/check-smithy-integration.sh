#!/bin/sh
set -eu

if [ "${AWS_SDK_BUILD_SMITHY_INTEGRATION:-0}" != "1" ]; then
    echo "smithy integration skipped: set AWS_SDK_BUILD_SMITHY_INTEGRATION=1"
    exit 0
fi

if ! command -v smithy >/dev/null 2>&1 && [ -z "${SMITHY_CLI:-}" ]; then
    echo "smithy integration unavailable: install Smithy CLI or set SMITHY_CLI"
    exit 0
fi

cargo check --manifest-path examples/generated-consumer/Cargo.toml

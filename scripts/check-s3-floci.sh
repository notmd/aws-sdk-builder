#!/bin/sh
set -eu

if [ "${AWS_SDK_BUILD_SKIP_FLOCI:-0}" = "1" ]; then
    echo "S3 Floci smoke test skipped by AWS_SDK_BUILD_SKIP_FLOCI=1"
    exit 0
fi

endpoint="${AWS_ENDPOINT_URL:-http://127.0.0.1:4566}"
region="${AWS_DEFAULT_REGION:-us-east-1}"
access_key="${AWS_ACCESS_KEY_ID:-test}"
secret_key="${AWS_SECRET_ACCESS_KEY:-test}"

case "${endpoint}" in
    http://127.0.0.1:*|https://127.0.0.1:*|http://localhost:*|https://localhost:*|http://[::1]:*|https://[::1]:*) ;;
    *)
        if [ "${ALLOW_NONLOCAL_FLOCI:-0}" != "1" ]; then
            echo "refusing non-loopback Floci endpoint: ${endpoint}" >&2
            echo "set ALLOW_NONLOCAL_FLOCI=1 only for an intentional override" >&2
            exit 2
        fi
        ;;
esac

http_code="$(curl -sS --max-time 3 -o /dev/null -w '%{http_code}' "${endpoint}/" 2>/dev/null || true)"
if [ "${http_code}" = "000" ]; then
    echo "Floci is unreachable at ${endpoint}" >&2
    echo "start the Docker emulator yourself, for example:" >&2
    echo "docker run --rm -d --name floci -p 4566:4566 floci/floci:latest" >&2
    exit 2
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_root=$(CDPATH= cd -- "${script_dir}/.." && pwd)
echo "Running Rust S3 Floci smoke test at ${endpoint}"
AWS_ENDPOINT_URL="${endpoint}" \
AWS_DEFAULT_REGION="${region}" \
AWS_ACCESS_KEY_ID="${access_key}" \
AWS_SECRET_ACCESS_KEY="${secret_key}" \
cargo run --manifest-path "${repo_root}/examples/floci-s3-smoke/Cargo.toml"

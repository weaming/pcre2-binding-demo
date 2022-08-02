#!/bin/bash

# macOS M1: /usr/local/Cellar/pcre2/10.40/include/pcre2.h (brew install pcre2)
: "${PCRE2SYS_HEADER:=/usr/include/pcre2.h}"

bindgen \
    "$PCRE2SYS_HEADER" \
    --ctypes-prefix '::libc' \
    --whitelist-function '^pcre2_.*' \
    --whitelist-type '^pcre2_.*' \
    --whitelist-var '^PCRE2_.*' \
    --blacklist-function '^.*_callout_.*' \
    --blacklist-type '^.*_callout_.*' \
    -- -DPCRE2_CODE_UNIT_WIDTH=8 > "./src/bindings.rs"
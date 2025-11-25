#!/bin/bash

set -e

FEATURE_SETS=(
    ""
    "assertions"
    "std"
    "std assertions"
)

WARNINGS=""

verify() {
    RUSTFLAGS=$1 cargo build --no-default-features --features "$2" $3
    RUSTFLAGS=$1 cargo test --no-default-features --features "$2" $3

    CARGO_CHECK_COLORLESS=$(RUSTFLAGS=$1 cargo check --no-default-features --features "$2" $3 2>&1)
    CARGO_CHECK_COLORED=$(RUSTFLAGS=$1 cargo check --no-default-features --features "$2" $3 --color=always 2>&1)

    if echo "$CARGO_CHECK_COLORLESS" | grep -q "warning:"; then
        WARNINGS+="$CARGO_CHECK_COLORED"$'\n'
    fi
}

for features in "${FEATURE_SETS[@]}"
do
    verify "-C overflow-checks=on" "$features overflow_checks"
    verify "-C overflow-checks=on" "$features overflow_checks" --release
    verify "-C overflow-checks=off" "$features"
    verify "-C overflow-checks=off" "$features" --release
done

if [ -n "$WARNINGS" ]; then
    echo -e "$WARNINGS"
    echo -e "\e[1;33mGG! verification passed (with warnings)\e[0m"
else
    echo -e "\e[1;32mGG! verification passed\e[0m"
fi

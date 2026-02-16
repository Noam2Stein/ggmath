#!/usr/bin/env bash

set -e

MAX_PROGRESS=57
INTEGRATIONS="bytemuck fixed fixp mint serde"

PROGRESS=0
WARNINGS=false

run() {
    echo
    echo "Commands: $PROGRESS/$MAX_PROGRESS"
    echo "$1"
    echo

    eval $1

    PROGRESS=$((PROGRESS+1))
    if echo "$(eval "$1 --color=never" 2>&1)" | grep -q "warning:"
    then
        WARNINGS=true
    fi
}

build() {
    local target=$1
    local profile=$2
    local backend=$3
    local assertions=$4
    local overflow_checks=$5

    local command=""

    command+="RUSTFLAGS=\"-C overflow-checks=$overflow_checks\" "
    command+="cargo clippy --no-default-features "
    command+="--features \"$backend $assertions\" "
    command+="--target $target "

    if [[ $profile == "release" ]]
    then
        command+="--release "
    fi

    # enable integration features based on an arbitrary condition to ensure that
    # the crate compiles both with and without them.
    if [[ $overflow_checks == "on" ]]
    then
        command+="--features \"$INTEGRATIONS\" "
    fi

    run "$command"
}

test() {
    local profile=$1
    local backend=$2
    local assertions=$3
    local overflow_checks=$4

    local command=""

    command+="RUSTFLAGS=\"-C overflow-checks=$overflow_checks\" "
    command+="cargo test --no-default-features "
    command+="--features \"$backend $assertions $INTEGRATIONS\" "

    if [[ $profile == "release" ]]
    then
        command+="--release "
    fi

    run "$command"
}

for target in "x86_64-unknown-linux-gnu" "riscv64gc-unknown-linux-gnu"
do
    for profile in "debug" "release"
    do
        for backend in "std" "libm" ""
        do
            for assertions in "assertions" "no-assertions"
            do
                for overflow_checks in "on" "off"
                do
                    build "$target" "$profile" "$backend" "$assertions" "$overflow_checks"
                done
            done
        done
    done
done

for profile in "debug" "release"
do
    for assertions in "assertions" "no-assertions"
    do
        for overflow_checks in "on" "off"
        do
            test "$profile" "std" "$assertions" "$overflow_checks"
        done
    done
done

test "release" "libm" "no-assertions" "off"

echo
echo "Commands: $PROGRESS/$MAX_PROGRESS"

if [[ "$WARNINGS" == true ]]
then
    echo -e "\e[1;33mGG. verification passed with warnings\e[0m"
else
    echo -e "\e[1;32mGG! verification passed\e[0m"
fi

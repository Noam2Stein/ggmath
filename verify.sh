set -e

FEATURE_SETS=(
    ""
    "right left up down forwards backwards std assert"
    "std assert"
    "right left up down forwards backwards std"
    "std"
    "right left up down forwards backwards std debug_assert"
    "std debug_assert"
    "right left up down forwards backwards assert"
    "right left forwards std assert"
)

for features in "${FEATURE_SETS[@]}"
do
    cargo build --no-default-features --features "$features"
    cargo test --no-default-features --features "$features"
done

echo -e "\e[1;32mGG! verification passed\e[0m"
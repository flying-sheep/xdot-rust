# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Build package
build:
    cargo hack --feature-powerset build --verbose

# Runs clippy on the sources
check:
    cargo hack --feature-powerset clippy --locked -- -D warnings

# Runs unit tests
test:
    cargo hack --feature-powerset test --locked

# Build documentation
doc:
    cargo +nightly doc --all-features

# Format code
fmt:
    cargo fmt

#!/bin/bash

set -e
cargo build --features all

cargo build --features num_traits_support,ux_support,unsafe_fetch
cargo build --features num_traits_support,ux_support,intx_support
cargo build --features num_traits_support,unsafe_fetch,intx_support
cargo build --features ux_support,unsafe_fetch,intx_support

cargo build --features num_traits_support,ux_support
cargo build --features num_traits_support,unsafe_fetch
cargo build --features num_traits_support,intx_support
cargo build --features intx_support,ux_support
cargo build --features unsafe_fetch,ux_support
cargo build --features intx_support,ux_support

cargo build --features num_traits_support
cargo build --features ux_support
cargo build --features intx_support
cargo build --features unsafe_fetch
cargo build --features half_support

cargo clean
cargo +nightly miri test --features all
cargo clean
cargo test --features all
cargo test
cargo test --release --features all
cargo test --release
cargo clean
cargo test --features num_traits_support,ux_support,intx_support,unsafe_fetch,half_support,serde
cargo clean
#s390x is big endian
cross build --target s390x-unknown-linux-gnu
cargo clean
cross build --target s390x-unknown-linux-gnu --features all
cargo clean
cross test --target s390x-unknown-linux-gnu --features all
cargo clean
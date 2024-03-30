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
cargo clean
cargo test --features all
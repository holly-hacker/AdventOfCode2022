default:
    @just --list

check:
    cargo check --benches --tests --no-default-features
    cargo check --benches --tests
    cargo clippy --benches --tests --no-default-features -- --warn clippy::nursery --warn clippy::pedantic
    cargo clippy --benches --tests -- --warn clippy::nursery --warn clippy::pedantic

bench feature='default':
    cargo test --no-default-features --features {{feature}} -q
    cargo bench --bench criterion --no-default-features --features {{feature}}

install-pgo:
    rustup component add llvm-tools-preview
    cargo install cargo-pgo

pgo feature='default':
    cargo pgo bench -- --bench criterion --no-default-features --features {{feature}} -- "real"
    cargo pgo optimize bench -- --bench criterion --no-default-features --features {{feature}} -- "real"

cachegrind feature='default':
    {{ if os_family() == "windows" { error("valgrind is not available on windows") } else {""} }}
    cargo test --no-default-features --features {{feature}} -q
    cargo bench --bench iai --no-default-features --features {{feature}}

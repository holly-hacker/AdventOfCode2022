default:
    @just --list

bench feature:
    cargo test --no-default-features --features {{feature}}
    cargo bench --no-default-features --features {{feature}}

install-pgo:
    rustup component add llvm-tools-preview
    cargo install cargo-pgo

pgo feature:
    cargo pgo bench -- --no-default-features --features {{feature}}
    cargo pgo optimize bench -- --no-default-features --features {{feature}}

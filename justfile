default:
    @just --list

bench feature:
    cargo test --no-default-features --features {{feature}}
    cargo bench --bench criterion --no-default-features --features {{feature}}

install-pgo:
    rustup component add llvm-tools-preview
    cargo install cargo-pgo

pgo feature:
    cargo pgo bench -- --bench criterion --no-default-features --features {{feature}}
    cargo pgo optimize bench -- --bench criterion --no-default-features --features {{feature}}

cachegrind feature:
    {{ if os_family() == "windows" { error("cachegrind is not available on windows") } else {""} }}
    cargo bench --bench iai --no-default-features --features {{feature}}

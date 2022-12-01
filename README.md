# Advent of Code 2022

## Running benchmarks
These commands require the [just command runner](https://just.systems). If you don't wish to
install this, adapt the commands from `justfile`.

Run standard benchmark:
```shell
just bench day01
```

Run PGO-optimized benchmark:
```shell
# once needed once
just install-pgo

just pgo day01
```

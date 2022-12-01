# Advent of Code 2022

## Running benchmarks
These commands require the [just command runner](https://just.systems). If you don't wish to
install this, adapt the commands from `justfile`.

### Standard benchmark
```shell
just bench day01
```

### PGO-optimized benchmark
```shell
# once needed once
just install-pgo

just pgo day01
```

### Cachegrind
Running benchmarks under cachegrind requires [valgrind](https://valgrind.org/), which is not
available on windows. If you use windows, you can use WSL to get access to a linux distro.

```shell
just cachegrind day01
```
# dsi-bitstream-py
![GitHub CI](https://github.com/zommiommy/dsi-bitstream-py/actions/workflows/test.yml/badge.svg)
![license](https://img.shields.io/crates/l/dsi-bitstream)
[![](https://tokei.rs/b1/github/zommiommy/dsi-bitstream-py?type=Rust,Python)](https://github.com/zommiommy/dsi-bitstream-py)
[![Supported Python versions](https://img.shields.io/badge/Python-3.7+-blue.svg)](https://pypi.org/project/ensmallen/#history)
[![Pypi total project downloads](https://pepy.tech/badge/dsi_bitstream)](https://pepy.tech/badge/dsi_bitstream)
[![Pypi project](https://badge.fury.io/py/dsi_bitstream.svg)](https://badge.fury.io/py/dsi_bitstream)

Python bindings for [dsi-bitstream-rs](https://github.com/vigna/dsi-bitstream-rs), a Rust implementation of read/write bit streams supporting several types of instantaneous codes.

## Installation
```
pip install dsi_bitstream
```

## Usage

### Reading and writing codes

```python
from dsi_bitstream import BitWriterLittleEndian, BitReaderLittleEndian

writer = BitWriterLittleEndian("./bitstream.bin")

# All write methods return the number of bits written.
writer.write_bits(10, n=5)         # write 10 as 5 raw bits
writer.write_unary(100)
writer.write_gamma(10)
writer.write_delta(2)
writer.write_omega(7)
writer.write_rice(3, k=4)
writer.write_golomb(4, b=10)
writer.write_zeta(10, k=3)
writer.write_pi(42, k=2)
writer.write_exp_golomb(100, k=3)
writer.write_minimal_binary(10, max=100)
writer.flush()

reader = BitReaderLittleEndian("./bitstream.bin")
assert reader.read_bits(n=5) == 10
assert reader.read_unary() == 100
assert reader.read_gamma() == 10
assert reader.read_delta() == 2
assert reader.read_omega() == 7
assert reader.read_rice(k=4) == 3
assert reader.read_golomb(b=10) == 4
assert reader.read_zeta(k=3) == 10
assert reader.read_pi(k=2) == 42
assert reader.read_exp_golomb(k=3) == 100
assert reader.read_minimal_binary(max=100) == 10
```

Seeking is supported on the reader:
```python
pos = reader.bit_pos()   # bits from the start of the file
reader.set_bit_pos(pos)  # seek back
```

Big-endian variants are available as `BitReaderBigEndian` / `BitWriterBigEndian`.

### Analyzing codes with `CodesStats`

`CodesStats` records a stream of non-negative integers and computes the total
bit cost for every supported code, so you can pick the most compact one:

```python
from dsi_bitstream import CodesStats

stats = CodesStats()
for value in data:
    stats.update(value)

# Best code and its total bit cost.
code, bits = stats.best_code()   # e.g. ("Zeta(3)", 48120)

# Full ranking, cheapest first.
for code, bits in stats.get_codes():
    print(f"{code:>20s}: {bits} bits")

# Query a specific code.
bits = stats.bits_for("Delta")   # returns None if out of tracked range

# Merge stats from parallel workers.
combined = stats_a + stats_b
```

Field-level access is available via properties: `total`, `unary`, `gamma`,
`delta`, `omega`, `vbyte`, `zeta`, `golomb`, `exp_golomb`, `rice`, `pi`.
The array properties (`zeta`, `golomb`, etc.) return a list of bit costs, one
per parameter value.

## Building

### With Nix (recommended)

The repository includes a `flake.nix` with two package outputs:

```shell
# Native wheel (linux tag, for local use)
nix build .#default.dist

# manylinux2014 wheel (PyPI-uploadable, uses zig as linker)
nix build .#manylinux.dist

# in either cases the wheel will be in:
ls result-dist/dsi_bitstream-*.whl
```

The manylinux wheel is built with `maturin --zig`, which links against glibc
2.17 headers shipped by zig, and verified with `auditwheel` during the build.

A dev shell is also available:
```shell
nix develop
maturin develop  # build & install in-place for development
```

### Without Nix

```shell
pip install maturin
maturin develop          # development build
maturin build --release  # release wheel
```

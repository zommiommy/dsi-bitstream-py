# dsi-bitstream-py
![GitHub CI](https://github.com/zommiommy/dsi-bitstream-py/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/crates/l/dsi-bitstream)
[![](https://tokei.rs/b1/github/zommiommy/dsi-bitstream-py?type=Rust,Python)](https://github.com/zommiommy/dsi-bitstream-py)
[![Supported Python versions](https://img.shields.io/badge/Python-3.7+-blue.svg)](https://pypi.org/project/ensmallen/#history)
[![Pypi total project downloads](https://pepy.tech/badge/dsi_bitstream)](https://pepy.tech/badge/dsi_bitstream)
[![Pypi project](https://badge.fury.io/py/dsi_bitstream.svg)](https://badge.fury.io/py/dsi_bitstream)

Python bindings of dsi-bitstream-rs

Install with:
```
pip install dsi_bitstream
```

Example:
```python
from dsi_bitstream import BitReaderLittleEndian, BitWriterLittleEndian

writer = BitWriterLittleEndian("./bitstream.bin")

# all write methods return how many bits were written
writer.write_bits(10, n = 5) # write 10 as 5 bits 
writer.write_unary(100)
writer.write_gamma(10)
writer.write_delta(2)
writer.write_rice(3, k=4)
writer.write_golomb(4, k=10)
writer.write_zeta(10, k=3)
writer.write_exp_golomb(100, k=3)
writer.write_minimal_binary(10, max=100)
writer.flush()

reader = BitReaderLittleEndian("./bitstream.bin")
assert reader.read_bits(n = 5) == 10
assert reader.read_unary() == 100
assert reader.read_gamma() == 10
print(reader.bit_pos()) # bits from the start of the file (here 497)
assert reader.read_delta() == 2
assert reader.read_rice(k=4) == 3
assert reader.read_golomb(k=10) == 4
assert reader.read_zeta(k=3) == 10
assert reader.read_exp_golomb(k=3) == 100
assert reader.read_minimal_binary(max=100) == 10

# the reader can seek in the file, while the writer cannot.
reader.set_bit_pos(497)
assert reader.read_delta() == 2
assert reader.read_rice(k=4) == 3
assert reader.read_golomb(k=10) == 4
assert reader.read_zeta(k=3) == 10
assert reader.read_exp_golomb(k=3) == 100
assert reader.read_minimal_binary(max=100) == 10
```


## Building
To publish on pypi for linux:
```shell
docker run --rm -v $PWD:/io ghcr.io/pyo3/maturin build --release
```

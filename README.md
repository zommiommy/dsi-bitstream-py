# dsi-bitstream-py
Python bindings of dsi-bitstream-rs



## Building
To publish on pypi for linux:
```shell
docker run --rm -v $PWD:/io ghcr.io/pyo3/maturin build --release
```
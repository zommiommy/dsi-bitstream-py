# Change Log

## [0.3.0] - 2026-04-07

### Added

* `Code` class wrapping the Rust `Codes` enum for dynamic code dispatch
  * Constructors: `Code.unary()`, `Code.gamma()`, `Code.delta()`, `Code.omega()`,
    `Code.vbyte_le()`, `Code.vbyte_be()`, `Code.zeta(k)`, `Code.pi(k)`,
    `Code.golomb(b)`, `Code.exp_golomb(k)`, `Code.rice(log2_b)`
  * `Code.parse(s)` -- construct from string (e.g. `"Zeta(3)"`)
  * `code.canonicalize()` -- canonical form (`Zeta(1)` -> `Gamma`, etc.)
  * `code.len(n)` -- bit length of encoding `n` under this code
  * `code.read(reader)` / `code.write(writer, n)` -- read/write using dynamic dispatch
  * `str()`, `repr()`, `==` (compares canonical forms)
* Code length functions (module-level, no stream needed):
  `len_unary`, `len_gamma`, `len_delta`, `len_omega`, `len_zeta`, `len_pi`,
  `len_rice`, `len_golomb`, `len_exp_golomb`, `len_minimal_binary`

## [0.2.0] - 2026-04-07

### Added

* Elias omega code (`read_omega` / `write_omega`)
* Pi code (`read_pi` / `write_pi`) — Apostolico-Drovandi pi codes
* `CodesStats` class for analyzing which instantaneous code best compresses a given stream of integers
* Nix flake with native and manylinux2014 wheel outputs (manylinux via zig, verified with auditwheel)

### Changed

* Updated dsi-bitstream to 0.9.1
* Updated pyo3 to 0.28.3
* Rust edition 2024
* `read_zeta` / `write_zeta`: parameter `k` type changed from `u64` to `usize` (matches upstream)
* `read_golomb` / `write_golomb`: parameter renamed from `k` to `b` for clarity


## [0.1.0] - 2024-05-25

### Changed

* First implementation based on dsi-bitstream-rs 0.4.2

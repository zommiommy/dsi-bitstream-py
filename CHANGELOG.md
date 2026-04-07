# Change Log

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

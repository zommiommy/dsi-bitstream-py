use ::dsi_bitstream::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

macro_rules! impl_all {
    ($reader:ident, $writer:ident, $endiannes:ty) => {
        #[pyclass]
        pub struct $reader {
            reader: BufBitReader<$endiannes, WordAdapter<u32, BufReader<File>>>,
        }

        #[pymethods]
        impl $reader {
            #[new]
            fn new(path: &str) -> PyResult<Self> {
                let file = File::open(path).map_err(|e| PyValueError::new_err(e))?;
                Ok(Self {
                    reader: BufBitReader::new(WordAdapter::new(BufReader::new(file))),
                })
            }

            fn read_bits(&mut self, n: usize) -> PyResult<u64> {
                self.reader
                    .read_bits(n)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn peek_bits(&mut self, n: usize) -> PyResult<u64> {
                self.reader
                    .peek_bits(n)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn skip_bits(&mut self, n: usize) -> PyResult<()> {
                self.reader
                    .skip_bits(n)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_unary(&mut self) -> PyResult<u64> {
                self.reader
                    .read_unary()
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_gamma(&mut self) -> PyResult<u64> {
                self.reader
                    .read_gamma()
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_delta(&mut self) -> PyResult<u64> {
                self.reader
                    .read_delta()
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_omega(&mut self) -> PyResult<u64> {
                self.reader
                    .read_omega()
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_rice(&mut self, k: usize) -> PyResult<u64> {
                self.reader
                    .read_rice(k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_golomb(&mut self, b: u64) -> PyResult<u64> {
                self.reader
                    .read_golomb(b)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_zeta(&mut self, k: usize) -> PyResult<u64> {
                self.reader
                    .read_zeta(k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_pi(&mut self, k: usize) -> PyResult<u64> {
                self.reader.read_pi(k).map_err(|e| PyValueError::new_err(e))
            }

            fn read_exp_golomb(&mut self, k: usize) -> PyResult<u64> {
                self.reader
                    .read_exp_golomb(k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn read_minimal_binary(&mut self, max: u64) -> PyResult<u64> {
                self.reader
                    .read_minimal_binary(max)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn bit_pos(&mut self) -> PyResult<u64> {
                self.reader.bit_pos().map_err(|e| PyValueError::new_err(e))
            }

            fn set_bit_pos(&mut self, bit_pos: u64) -> PyResult<()> {
                self.reader
                    .set_bit_pos(bit_pos)
                    .map_err(|e| PyValueError::new_err(e))
            }
        }

        #[pyclass]
        pub struct $writer {
            writer: BufBitWriter<$endiannes, WordAdapter<u32, BufWriter<File>>>,
        }

        #[pymethods]
        impl $writer {
            #[new]
            fn new(path: &str) -> PyResult<Self> {
                let file = File::create(path).map_err(|e| PyValueError::new_err(e))?;
                Ok(Self {
                    writer: BufBitWriter::new(WordAdapter::new(BufWriter::new(file))),
                })
            }

            fn write_bits(&mut self, value: u64, n: usize) -> PyResult<usize> {
                self.writer
                    .write_bits(value, n)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_unary(&mut self, value: u64) -> PyResult<usize> {
                self.writer
                    .write_unary(value)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_gamma(&mut self, value: u64) -> PyResult<usize> {
                self.writer
                    .write_gamma(value)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_delta(&mut self, value: u64) -> PyResult<usize> {
                self.writer
                    .write_delta(value)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_omega(&mut self, value: u64) -> PyResult<usize> {
                self.writer
                    .write_omega(value)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_rice(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer
                    .write_rice(value, k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_golomb(&mut self, value: u64, b: u64) -> PyResult<usize> {
                self.writer
                    .write_golomb(value, b)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_zeta(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer
                    .write_zeta(value, k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_pi(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer
                    .write_pi(value, k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_exp_golomb(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer
                    .write_exp_golomb(value, k)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn write_minimal_binary(&mut self, value: u64, max: u64) -> PyResult<usize> {
                self.writer
                    .write_minimal_binary(value, max)
                    .map_err(|e| PyValueError::new_err(e))
            }

            fn flush(&mut self) -> PyResult<usize> {
                self.writer.flush().map_err(|e| PyValueError::new_err(e))
            }
        }
    };
}

impl_all!(BitReaderLittleEndian, BitWriterLittleEndian, LE);
impl_all!(BitReaderBigEndian, BitWriterBigEndian, BE);

/// Analyzes a stream of integers to determine which code provides the best
/// compression. Feed values via `update` / `update_many`, then inspect
/// `best_code` or `get_codes`.
#[pyclass]
#[pyo3(name = "CodesStats")]
pub struct PyCodesStats {
    inner: CodesStats,
}

#[pymethods]
impl PyCodesStats {
    #[new]
    fn new() -> Self {
        Self {
            inner: CodesStats::default(),
        }
    }

    /// Record one occurrence of `n`.
    fn update(&mut self, n: u64) -> u64 {
        self.inner.update(n)
    }

    /// Record `count` occurrences of `n`.
    fn update_many(&mut self, n: u64, count: u64) -> u64 {
        self.inner.update_many(n, count)
    }

    /// Return `(code_name, total_bits)` for the best code.
    fn best_code(&self) -> (String, u64) {
        let (code, bits) = self.inner.best_code();
        (code.to_string(), bits)
    }

    /// Return a list of `(code_name, total_bits)` sorted by ascending bit cost.
    fn get_codes(&self) -> Vec<(String, u64)> {
        self.inner
            .get_codes()
            .into_iter()
            .map(|(code, bits)| (code.to_string(), bits))
            .collect()
    }

    /// Return the total bits for a given code, or `None` if the code/parameter
    /// is outside the tracked range.
    fn bits_for(&self, code_name: &str) -> PyResult<Option<u64>> {
        let code: Codes = code_name
            .parse()
            .map_err(|e: CodeError| PyValueError::new_err(e.to_string()))?;
        Ok(self.inner.bits_for(code))
    }

    /// Merge another `CodesStats` into this one.
    fn add(&mut self, other: &PyCodesStats) {
        self.inner.add(&other.inner);
    }

    fn __iadd__(&mut self, other: &PyCodesStats) {
        self.inner.add(&other.inner);
    }

    fn __add__(&self, other: &PyCodesStats) -> PyCodesStats {
        let mut result = self.inner;
        result.add(&other.inner);
        PyCodesStats { inner: result }
    }

    fn __repr__(&self) -> String {
        let (code, bits) = self.inner.best_code();
        format!(
            "CodesStats(total={}, best=({}, {}))",
            self.inner.total, code, bits
        )
    }

    // --- Field accessors ---

    #[getter]
    fn total(&self) -> u64 {
        self.inner.total
    }

    #[getter]
    fn unary(&self) -> u64 {
        self.inner.unary
    }

    #[getter]
    fn gamma(&self) -> u64 {
        self.inner.gamma
    }

    #[getter]
    fn delta(&self) -> u64 {
        self.inner.delta
    }

    #[getter]
    fn omega(&self) -> u64 {
        self.inner.omega
    }

    #[getter]
    fn vbyte(&self) -> u64 {
        self.inner.vbyte
    }

    /// Zeta code costs. Index 0 = zeta_1, index 1 = zeta_2, etc.
    #[getter]
    fn zeta(&self) -> Vec<u64> {
        self.inner.zeta.to_vec()
    }

    /// Golomb code costs. Index 0 = Golomb(1), index 1 = Golomb(2), etc.
    #[getter]
    fn golomb(&self) -> Vec<u64> {
        self.inner.golomb.to_vec()
    }

    /// Exponential Golomb code costs. Index 0 = ExpGolomb(0), index 1 = ExpGolomb(1), etc.
    #[getter]
    fn exp_golomb(&self) -> Vec<u64> {
        self.inner.exp_golomb.to_vec()
    }

    /// Rice code costs. Index 0 = Rice(0), index 1 = Rice(1), etc.
    #[getter]
    fn rice(&self) -> Vec<u64> {
        self.inner.rice.to_vec()
    }

    /// Pi code costs. Index 0 = Pi(2), index 1 = Pi(3), etc.
    #[getter]
    fn pi(&self) -> Vec<u64> {
        self.inner.pi.to_vec()
    }
}

// --- Code length functions (pure, no stream needed) ---

/// Returns the length in bits of the unary code for `n`.
#[pyfunction]
fn len_unary(n: u64) -> usize {
    n as usize + 1
}

/// Returns the length in bits of the γ code for `n`.
#[pyfunction]
#[pyo3(name = "len_gamma")]
fn py_len_gamma(n: u64) -> usize {
    ::dsi_bitstream::codes::len_gamma(n)
}

/// Returns the length in bits of the δ code for `n`.
#[pyfunction]
#[pyo3(name = "len_delta")]
fn py_len_delta(n: u64) -> usize {
    ::dsi_bitstream::codes::len_delta(n)
}

/// Returns the length in bits of the ω code for `n`.
#[pyfunction]
#[pyo3(name = "len_omega")]
fn py_len_omega(n: u64) -> usize {
    ::dsi_bitstream::codes::len_omega(n)
}

/// Returns the length in bits of the ζ code for `n` with parameter `k`.
#[pyfunction]
#[pyo3(name = "len_zeta")]
fn py_len_zeta(n: u64, k: usize) -> usize {
    ::dsi_bitstream::codes::len_zeta(n, k)
}

/// Returns the length in bits of the π code for `n` with parameter `k`.
#[pyfunction]
#[pyo3(name = "len_pi")]
fn py_len_pi(n: u64, k: usize) -> usize {
    ::dsi_bitstream::codes::len_pi(n, k)
}

/// Returns the length in bits of the Rice code for `n` with parameter `log2_b`.
#[pyfunction]
#[pyo3(name = "len_rice")]
fn py_len_rice(n: u64, log2_b: usize) -> usize {
    ::dsi_bitstream::codes::len_rice(n, log2_b)
}

/// Returns the length in bits of the Golomb code for `n` with parameter `b`.
#[pyfunction]
#[pyo3(name = "len_golomb")]
fn py_len_golomb(n: u64, b: u64) -> usize {
    ::dsi_bitstream::codes::len_golomb(n, b)
}

/// Returns the length in bits of the exponential Golomb code for `n` with parameter `k`.
#[pyfunction]
#[pyo3(name = "len_exp_golomb")]
fn py_len_exp_golomb(n: u64, k: usize) -> usize {
    ::dsi_bitstream::codes::len_exp_golomb(n, k)
}

/// Returns the length in bits of the minimal binary code for `n` with upper bound `max`.
#[pyfunction]
#[pyo3(name = "len_minimal_binary")]
fn py_len_minimal_binary(n: u64, max: u64) -> usize {
    ::dsi_bitstream::codes::len_minimal_binary(n, max)
}

/// A code descriptor. Wraps the Rust `Codes` enum so you can refer to a
/// specific instantaneous code by name and optional parameter.
///
/// Construct via the named class methods:
///
///     c = Code.gamma()
///     c = Code.zeta(3)
///
/// `Code` supports `str()`, `repr()`, `==`, and parsing from strings:
///
///     c = Code.parse("Zeta(3)")
///
/// Use `len(code, n)` to compute the length of the code for value `n`
/// without writing to a stream.
#[pyclass(from_py_object)]
#[pyo3(name = "Code")]
#[derive(Clone)]
pub struct PyCode {
    inner: Codes,
}

#[pymethods]
impl PyCode {
    // --- Constructors (one per variant) ---

    #[staticmethod]
    fn unary() -> Self {
        Self {
            inner: Codes::Unary,
        }
    }

    #[staticmethod]
    fn gamma() -> Self {
        Self {
            inner: Codes::Gamma,
        }
    }

    #[staticmethod]
    fn delta() -> Self {
        Self {
            inner: Codes::Delta,
        }
    }

    #[staticmethod]
    fn omega() -> Self {
        Self {
            inner: Codes::Omega,
        }
    }

    #[staticmethod]
    fn vbyte_le() -> Self {
        Self {
            inner: Codes::VByteLe,
        }
    }

    #[staticmethod]
    fn vbyte_be() -> Self {
        Self {
            inner: Codes::VByteBe,
        }
    }

    #[staticmethod]
    fn zeta(k: usize) -> Self {
        Self {
            inner: Codes::Zeta(k),
        }
    }

    #[staticmethod]
    fn pi(k: usize) -> Self {
        Self {
            inner: Codes::Pi(k),
        }
    }

    #[staticmethod]
    fn golomb(b: u64) -> Self {
        Self {
            inner: Codes::Golomb(b),
        }
    }

    #[staticmethod]
    fn exp_golomb(k: usize) -> Self {
        Self {
            inner: Codes::ExpGolomb(k),
        }
    }

    #[staticmethod]
    fn rice(log2_b: usize) -> Self {
        Self {
            inner: Codes::Rice(log2_b),
        }
    }

    /// Parse a code from its string representation (e.g. "Gamma", "Zeta(3)").
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        let code: Codes = s
            .parse()
            .map_err(|e: CodeError| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: code })
    }

    /// Return the canonical form of this code.
    ///
    /// Equivalent codes collapse: e.g. `Zeta(1)` -> `Gamma`,
    /// `Rice(0)` -> `Unary`, `Golomb(2**n)` -> `Rice(n)`.
    fn canonicalize(&self) -> Self {
        Self {
            inner: self.inner.canonicalize(),
        }
    }

    /// Return the length in bits that `n` would occupy under this code.
    fn len(&self, n: u64) -> usize {
        CodeLen::len(&self.inner, n)
    }

    /// Read a value from the given reader using this code.
    fn read(&self, reader: &Bound<'_, PyAny>) -> PyResult<u64> {
        if let Ok(r) = reader.cast::<BitReaderLittleEndian>() {
            self.inner
                .read::<LE, _>(&mut r.borrow_mut().reader)
                .map_err(|e| PyValueError::new_err(e))
        } else if let Ok(r) = reader.cast::<BitReaderBigEndian>() {
            self.inner
                .read::<BE, _>(&mut r.borrow_mut().reader)
                .map_err(|e| PyValueError::new_err(e))
        } else {
            Err(PyValueError::new_err(
                "expected BitReaderLittleEndian or BitReaderBigEndian",
            ))
        }
    }

    /// Write a value to the given writer using this code.
    /// Returns the number of bits written.
    fn write(&self, writer: &Bound<'_, PyAny>, n: u64) -> PyResult<usize> {
        if let Ok(w) = writer.cast::<BitWriterLittleEndian>() {
            self.inner
                .write::<LE, _>(&mut w.borrow_mut().writer, n)
                .map_err(|e| PyValueError::new_err(e))
        } else if let Ok(w) = writer.cast::<BitWriterBigEndian>() {
            self.inner
                .write::<BE, _>(&mut w.borrow_mut().writer, n)
                .map_err(|e| PyValueError::new_err(e))
        } else {
            Err(PyValueError::new_err(
                "expected BitWriterLittleEndian or BitWriterBigEndian",
            ))
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Code.parse(\"{}\")", self.inner)
    }

    fn __eq__(&self, other: &PyCode) -> bool {
        // Compare canonical forms so equivalent codes are equal.
        self.inner.canonicalize() == other.inner.canonicalize()
    }
}

/// Bindings for dsi-bitstream-rs
#[pymodule]
fn dsi_bitstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitReaderLittleEndian>()?;
    m.add_class::<BitReaderBigEndian>()?;
    m.add_class::<BitWriterLittleEndian>()?;
    m.add_class::<BitWriterBigEndian>()?;
    m.add_class::<PyCodesStats>()?;
    m.add_class::<PyCode>()?;
    m.add_function(wrap_pyfunction!(len_unary, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_gamma, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_delta, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_omega, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_zeta, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_pi, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_rice, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_golomb, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_exp_golomb, m)?)?;
    m.add_function(wrap_pyfunction!(py_len_minimal_binary, m)?)?;
    Ok(())
}

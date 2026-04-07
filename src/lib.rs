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
                self.reader
                    .read_pi(k)
                    .map_err(|e| PyValueError::new_err(e))
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

/// Bindings for dsi-bitstream-rs
#[pymodule]
fn dsi_bitstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitReaderLittleEndian>()?;
    m.add_class::<BitReaderBigEndian>()?;
    m.add_class::<BitWriterLittleEndian>()?;
    m.add_class::<BitWriterBigEndian>()?;
    m.add_class::<PyCodesStats>()?;
    Ok(())
}

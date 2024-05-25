use ::dsi_bitstream::prelude::*;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
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
                self.reader.read_bits(n).map_err(|e| PyValueError::new_err(e))
            }
        
            fn peek_bits(&mut self, n: usize) -> PyResult<u64> {
                self.reader.peek_bits(n).map_err(|e| PyValueError::new_err(e))
            }
        
            fn skip_bits(&mut self, n: usize) -> PyResult<()> {
                self.reader.skip_bits(n).map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_unary(&mut self) -> PyResult<u64> {
                self.reader.read_unary().map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_gamma(&mut self) -> PyResult<u64> {
                self.reader.read_gamma().map_err(|e| PyValueError::new_err(e))
            }
            
            fn read_delta(&mut self) -> PyResult<u64> {
                self.reader.read_delta().map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_rice(&mut self, k: usize) -> PyResult<u64> {
                self.reader.read_rice(k).map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_golomb(&mut self, k: u64) -> PyResult<u64> {
                self.reader.read_golomb(k).map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_zeta(&mut self, k: u64) -> PyResult<u64> {
                self.reader.read_zeta(k).map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_exp_golomb(&mut self, k: usize) -> PyResult<u64> {
                self.reader.read_exp_golomb(k).map_err(|e| PyValueError::new_err(e))
            }
        
            fn read_minimal_binary(&mut self, max: u64) -> PyResult<u64> {
                self.reader.read_minimal_binary(max).map_err(|e| PyValueError::new_err(e))
            }
        
            fn bit_pos(&mut self) -> PyResult<u64> {
                self.reader.bit_pos().map_err(|e| PyValueError::new_err(e))
            }
        
            fn set_bit_pos(&mut self, bit_pos: u64) -> PyResult<()> {
                self.reader.set_bit_pos(bit_pos).map_err(|e| PyValueError::new_err(e))
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
                self.writer.write_bits(value, n).map_err(|e| PyValueError::new_err(e))
            }

            fn write_unary(&mut self, value: u64) -> PyResult<usize> {
                self.writer.write_unary(value).map_err(|e| PyValueError::new_err(e))
            }

            fn write_gamma(&mut self, value: u64) -> PyResult<usize> {
                self.writer.write_gamma(value).map_err(|e| PyValueError::new_err(e))
            }

            fn write_delta(&mut self, value: u64) -> PyResult<usize> {
                self.writer.write_delta(value).map_err(|e| PyValueError::new_err(e))
            }

            fn write_rice(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer.write_rice(value, k).map_err(|e| PyValueError::new_err(e))
            }

            fn write_golomb(&mut self, value: u64, k: u64) -> PyResult<usize> {
                self.writer.write_golomb(value, k).map_err(|e| PyValueError::new_err(e))
            }

            fn write_zeta(&mut self, value: u64, k: u64) -> PyResult<usize> {
                self.writer.write_zeta(value, k).map_err(|e| PyValueError::new_err(e))
            }

            fn write_exp_golomb(&mut self, value: u64, k: usize) -> PyResult<usize> {
                self.writer.write_exp_golomb(value, k).map_err(|e| PyValueError::new_err(e))
            }

            fn write_minimal_binary(&mut self, value: u64, max: u64) -> PyResult<usize> {
                self.writer.write_minimal_binary(value, max).map_err(|e| PyValueError::new_err(e))
            }

            fn flush(&mut self) -> PyResult<usize> {
                self.writer.flush().map_err(|e| PyValueError::new_err(e))
            }
        }
    };
}

impl_all!(BitReaderLittleEndian, BitWriterLittleEndian, LE);
impl_all!(BitReaderBigEndian, BitWriterBigEndian, BE);

/// Bindings for dsi-bitstream-rs
#[pymodule]
fn dsi_bitstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitReaderLittleEndian>()?;
    m.add_class::<BitReaderBigEndian>()?;
    m.add_class::<BitWriterLittleEndian>()?;
    m.add_class::<BitWriterBigEndian>()?;
    Ok(())
}
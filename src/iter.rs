#![feature(specialization)]
use pyo3::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use pyo3::PyIterProtocol;

#[pyclass]
pub struct RowReader {
    path: Box<Path>,
}

#[pyclass]
pub struct RowIterator {
    file: BufReader<File>,
}

use std::error::Error;
fn convert_result<T>(res: Result<T, impl Error>) -> PyResult<T> {
    res.map_err(|e| PyErr::new::<pyo3::exceptions::RuntimeError, _>(format!("{}", e)))
}

#[pymethods]
impl RowReader {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        use std::path::PathBuf;
        let path: Box<Path> = PathBuf::from(path).into();
        convert_result(Ok::<Self, std::io::Error>(Self { path }))
    }
}

#[pyproto]
impl PyIterProtocol for RowReader {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<RowIterator> {
        Ok(RowIterator {
            file: BufReader::new(File::open(slf.path.clone())?),
        })
    }
}

#[pyproto]
impl PyIterProtocol for RowIterator {
    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<String>> {
        let mut s = String::new();
        let l = slf.file.read_line(&mut s)?;
        if l == 0 {
            Ok(None)
        } else {
            if s.chars().last().unwrap() == '\n' {
                s.pop();
                if s.chars().last().unwrap() == '\r' {
                    s.pop();
                }
            }
            Ok(Some(s))
        }
    }
}

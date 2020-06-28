use async_std::fs::File;
use async_std::path::Path;
use async_std::sync::Arc;
use pyo3::prelude::*;
use std::error::Error;

mod iter;

fn convert_result<T>(res: Result<T, impl Error>) -> PyResult<T> {
    res.map_err(|e| PyErr::new::<pyo3::exceptions::RuntimeError, _>(format!("{}", e)))
}

/*
#[pyclass]
pub struct CSVLoader {
    path: Option<String>,
    file: Option<File>,
}

impl CSVLoader {
    async fn _open(&mut self) -> Result<(), std::io::Error> {
        let p = self.path.clone().unwrap();
        self.file = Some(File::open(p).await?);
        Ok(())
    }
    async fn py_open(&mut self) -> PyResult<()> {
        convert_result(self._open().await)
    }
}

#[pymethods]
impl CSVLoader {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        convert_result(Ok::<Self, std::io::Error>(Self {
            path: Some(path),
            file: None,
        }))
    }

    fn open() -> PyResult<()> {}

    fn gen_async(&self) -> pyo3::ffi::PyAsyncMethods {
        let t = Box::pin(self._open()).as_mut();
        pyo3::ffi::PyAsyncMethods {
            am_aiter: None,
            am_anext: t.poll(1),
            am_await: None,
        }
    }

    fn length(&self) -> i8 {
        0
    }
}
*/

#[pymodule]
fn pyon(_py: Python, m: &PyModule) -> PyResult<()> {
    // _py = _py;
    // m.add_class::<CSVLoader>()?;
    m.add_class::<iter::RowReader>()?;
    m.add_class::<iter::RowIterator>()?;
    Ok(())
}

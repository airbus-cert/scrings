use std::io::{Read, Seek};
use std::io;
use pyo3::prelude::*;
use pyo3_file::PyFileLikeObject;
use scrings::ps::Powershell;
use scrings::python::Python;
use scrings::php::Php;
use scrings::js::Javascript;
use scrings::bash::Bash;
use scrings::sql::Sql;
use scrings::strings::Utf16le;
use scrings::parser::{IterScrings,LanguageIterator};

pub trait ReadSeek: Read + Seek + Send + Sync + 'static
{
    fn tell(&mut self) -> io::Result<u64> {
        self.stream_position()
    }
}

impl<T: Read + Seek + Send + Sync + 'static> ReadSeek for T {}

#[pyclass]
struct PowershellItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, Utf16le, Powershell>
}

#[pymethods]
impl PowershellItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn powershell(file_like: PyObject, step: Option<usize>) -> PyResult<PowershellItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, true) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(PowershellItemIterator { iter: buffer.iter_scrings::<Utf16le, Powershell>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

#[pyclass]
struct PythonItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, u8, Python>
}

#[pymethods]
impl PythonItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn python(file_like: PyObject, step: Option<usize>) -> PyResult<PythonItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, false) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(PythonItemIterator { iter: buffer.iter_scrings::<u8, Python>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

#[pyclass]
struct PhpItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, u8, Php>
}

#[pymethods]
impl PhpItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn php(file_like: PyObject, step: Option<usize>) -> PyResult<PhpItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, false) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(PhpItemIterator { iter: buffer.iter_scrings::<u8, Php>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

#[pyclass]
struct JavascriptItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, u8, Javascript>
}

#[pymethods]
impl JavascriptItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn javascript(file_like: PyObject, step: Option<usize>) -> PyResult<JavascriptItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, false) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(JavascriptItemIterator { iter: buffer.iter_scrings::<u8, Javascript>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

#[pyclass]
struct BashItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, u8, Bash>
}

#[pymethods]
impl BashItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn bash(file_like: PyObject, step: Option<usize>) -> PyResult<BashItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, false) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(BashItemIterator { iter: buffer.iter_scrings::<u8, Bash>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

#[pyclass]
struct SqlItemIterator {
    iter: LanguageIterator<Box<dyn ReadSeek>, u8, Sql>
}

#[pymethods]
impl SqlItemIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(u64, String)> {
        slf.iter.next().clone()
    }
}

#[pyfunction]
#[pyo3(signature = (file_like, step=None))]
fn sql(file_like: PyObject, step: Option<usize>) -> PyResult<SqlItemIterator> {
    match PyFileLikeObject::with_requirements(file_like, true, false, true, false) {
        Ok(f) => {
            let buffer = Box::new(f) as Box<dyn ReadSeek>;
            return Ok(SqlItemIterator { iter: buffer.iter_scrings::<u8, Sql>(step.unwrap_or(20))})
        }
        Err(e) => return Err(e),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyscrings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(powershell, m)?)?;
    m.add_function(wrap_pyfunction!(python, m)?)?;
    m.add_function(wrap_pyfunction!(php, m)?)?;
    m.add_function(wrap_pyfunction!(javascript, m)?)?;
    m.add_function(wrap_pyfunction!(bash, m)?)?;
    m.add_function(wrap_pyfunction!(sql, m)?)?;
    Ok(())
}

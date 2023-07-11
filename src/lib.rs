#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use geohash;
use pyo3::{create_exception, exceptions::PyValueError, prelude::*};


create_exception!(_geohashr, DecodeError, PyValueError, "Geohash decode error");
create_exception!(_geohashr, EncodeError, PyValueError, "Geohash encode error");

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode(py: Python, hash: &str) -> PyResult<(f64, f64)> {
    match py.allow_threads(|| {
        geohash::decode(hash)
    }) {
        Ok((coords, _, _)) => Ok((coords.x, coords.y)),
        Err(err) => Err(DecodeError::new_err(err.to_string()))
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode_exact(py: Python, hash: &str) -> PyResult<(f64, f64, f64, f64)> {
    match py.allow_threads(|| {
        geohash::decode(hash)
    }) {
        Ok((coords, dx, dy)) => Ok((coords.x, coords.y, dx, dy)),
        Err(err) => Err(DecodeError::new_err(err.to_string()))
    }
}

#[pyfunction]
#[pyo3(signature = (lat, lon, len=12))]
fn encode(py: Python, lat: f64, lon: f64, len: usize) -> PyResult<String> {
    match py.allow_threads(|| {
        geohash::encode(geohash::Coord { x: lat, y: lon }, len)
    }) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(EncodeError::new_err(err.to_string()))
    }
}

#[pymodule]
fn _geohashr(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(decode, module)?).unwrap();
    module.add_function(wrap_pyfunction!(decode_exact, module)?).unwrap();
    module.add_function(wrap_pyfunction!(encode, module)?).unwrap();

    #[cfg(not(PyPy))]
    pyo3::prepare_freethreaded_python();

    Ok(())
}

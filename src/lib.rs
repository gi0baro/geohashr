#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::collections::HashMap;

use geohash::{self, Direction};
use pyo3::{
    create_exception,
    exceptions::PyValueError,
    prelude::*,
    types::{IntoPyDict, PyDict},
};

create_exception!(_geohashr, DecodeError, PyValueError, "Geohash decode error");
create_exception!(_geohashr, EncodeError, PyValueError, "Geohash encode error");
create_exception!(_geohashr, InvalidDirectionError, PyValueError, "Can't decode coordinate");

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode(py: Python, hash: &str) -> PyResult<(f64, f64)> {
    match py.allow_threads(|| geohash::decode(hash)) {
        Ok((coords, _, _)) => Ok((coords.x, coords.y)),
        Err(err) => Err(DecodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode_exact(py: Python, hash: &str) -> PyResult<(f64, f64, f64, f64)> {
    match py.allow_threads(|| geohash::decode(hash)) {
        Ok((coords, dx, dy)) => Ok((coords.x, coords.y, dx, dy)),
        Err(err) => Err(DecodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (lat, lon, len=12))]
fn encode(py: Python, lat: f64, lon: f64, len: usize) -> PyResult<String> {
    match py.allow_threads(|| geohash::encode(geohash::Coord { x: lat, y: lon }, len)) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn bbox<'p>(py: Python<'p>, hash: &str) -> PyResult<&'p PyDict> {
    match py.allow_threads(|| geohash::decode_bbox(hash)) {
        Ok(bbox) => Ok([
            ("e", bbox.min().x), 
            ("s", bbox.min().y), 
            ("w", bbox.max().x),
            ("n", bbox.max().y)].into_py_dict(py)),
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature=(hash))]
fn neighbors<'p>(py: Python<'p>, hash: &str) -> PyResult<&'p PyDict> {
    match py.allow_threads(|| geohash::neighbors(hash)) {
        Ok(neighbors) => {
            let map = [
                ("sw", neighbors.sw),
                ("s", neighbors.s),
                ("se", neighbors.se),
                ("w", neighbors.w),
                ("e", neighbors.e),
                ("nw", neighbors.nw),
                ("n", neighbors.n),
                ("ne", neighbors.ne),
            ]
            .into_py_dict(py);
            Ok(map)
        }
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature=(hash, direction))]
fn neighbor(py: Python, hash: &str, direction: &str) -> PyResult<String> {
    let directions = HashMap::from(
        [
            ("sw",  Direction::SW),
            ("s",  Direction::S),
            ("se",  Direction::SE),
            ("w",  Direction::W),
            ("e",  Direction::E),
            ("nw",  Direction::NW),
            ("n",  Direction::N),
            ("ne",  Direction::NE),
        ]
    );
    match py.allow_threads(|| {
        if let Some(dir) = directions.get(direction) {
        match geohash::neighbor(hash, *dir) {
            Ok(hash) => Ok(hash),
            Err(err) => Err(EncodeError::new_err(err.to_string()))
        }
    }
    else {
        Err(InvalidDirectionError::new_err("Invalid direction"))
    }
    }) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(err),
    }
}

#[pymodule]
fn _geohashr(_py: Python, module: &PyModule) -> PyResult<()> {
    module
        .add_function(wrap_pyfunction!(decode, module)?)
        .unwrap();
    module
        .add_function(wrap_pyfunction!(decode_exact, module)?)
        .unwrap();
    module
        .add_function(wrap_pyfunction!(encode, module)?)
        .unwrap();
    module
        .add_function(wrap_pyfunction!(bbox, module)?)
        .unwrap();
    module
        .add_function(wrap_pyfunction!(neighbors, module)?)
        .unwrap();
    module
        .add_function(wrap_pyfunction!(neighbor, module)?)
        .unwrap();
    module.add("DecodeError", _py.get_type::<DecodeError>())?;    
    module.add("EncodeError", _py.get_type::<EncodeError>())?; 
    module.add("InvalidDirectionError", _py.get_type::<InvalidDirectionError>())?;    

    #[cfg(not(PyPy))]
    pyo3::prepare_freethreaded_python();

    Ok(())
}

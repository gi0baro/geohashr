#[cfg(not(any(target_os = "freebsd", target_os = "windows")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(any(target_os = "freebsd", target_os = "windows"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use geohash::{self, Direction};
use lazy_static::lazy_static;
use pyo3::{
    create_exception,
    exceptions::{PySyntaxError, PyValueError},
    prelude::*,
    types::PyDict,
};
use std::{collections::HashMap, sync::OnceLock};

create_exception!(_geohashr, DecodeError, PyValueError, "Geohash decode error");
create_exception!(_geohashr, EncodeError, PyValueError, "Geohash encode error");
create_exception!(_geohashr, ParamError, PySyntaxError, "Geohash parameter error");

enum NeighborError {
    Hash(geohash::GeohashError),
    Direction,
}

lazy_static! {
    static ref DIRECTION_MAP: HashMap<&'static str, Direction> = HashMap::from([
        ("sw", Direction::SW),
        ("s", Direction::S),
        ("se", Direction::SE),
        ("w", Direction::W),
        ("e", Direction::E),
        ("nw", Direction::NW),
        ("n", Direction::N),
        ("ne", Direction::NE)
    ]);
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode(py: Python, hash: &str) -> PyResult<(f64, f64)> {
    match py.allow_threads(|| geohash::decode(hash)) {
        Ok((coords, _, _)) => Ok((coords.y, coords.x)),
        Err(err) => Err(DecodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn decode_exact(py: Python, hash: &str) -> PyResult<(f64, f64, f64, f64)> {
    match py.allow_threads(|| geohash::decode(hash)) {
        Ok((coords, dx, dy)) => Ok((coords.y, coords.x, dy, dx)),
        Err(err) => Err(DecodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (lat, lon, len = 12))]
fn encode(py: Python, lat: f64, lon: f64, len: usize) -> PyResult<String> {
    match py.allow_threads(|| geohash::encode(geohash::Coord { x: lon, y: lat }, len)) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn bbox<'p>(py: Python<'p>, hash: &str) -> PyResult<Bound<'p, PyDict>> {
    match py.allow_threads(|| {
        let bbox = geohash::decode_bbox(hash)?;
        Ok::<(geohash::Coord, geohash::Coord), geohash::GeohashError>((bbox.min(), bbox.max()))
    }) {
        Ok((min, max)) => {
            let dict = PyDict::new_bound(py);
            dict.set_item(pyo3::intern!(py, "e"), max.x)?;
            dict.set_item(pyo3::intern!(py, "s"), min.y)?;
            dict.set_item(pyo3::intern!(py, "w"), min.x)?;
            dict.set_item(pyo3::intern!(py, "n"), max.y)?;
            Ok(dict)
        }
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash))]
fn neighbors<'p>(py: Python<'p>, hash: &str) -> PyResult<Bound<'p, PyDict>> {
    match py.allow_threads(|| geohash::neighbors(hash)) {
        Ok(neighbors) => {
            let dict = PyDict::new_bound(py);
            dict.set_item(pyo3::intern!(py, "e"), neighbors.e)?;
            dict.set_item(pyo3::intern!(py, "n"), neighbors.n)?;
            dict.set_item(pyo3::intern!(py, "ne"), neighbors.ne)?;
            dict.set_item(pyo3::intern!(py, "nw"), neighbors.nw)?;
            dict.set_item(pyo3::intern!(py, "s"), neighbors.s)?;
            dict.set_item(pyo3::intern!(py, "se"), neighbors.se)?;
            dict.set_item(pyo3::intern!(py, "sw"), neighbors.sw)?;
            dict.set_item(pyo3::intern!(py, "w"), neighbors.w)?;
            Ok(dict)
        }
        Err(err) => Err(EncodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (hash, direction))]
fn neighbor(py: Python, hash: &str, direction: &str) -> PyResult<String> {
    match py.allow_threads(|| {
        if let Some(dir) = DIRECTION_MAP.get(direction) {
            geohash::neighbor(hash, *dir).map_err(NeighborError::Hash)
        } else {
            Err(NeighborError::Direction)
        }
    }) {
        Ok(hash) => Ok(hash),
        Err(NeighborError::Hash(err)) => Err(EncodeError::new_err(err.to_string())),
        Err(NeighborError::Direction) => Err(ParamError::new_err("Invalid direction")),
    }
}

pub fn get_lib_version() -> &'static str {
    static LIB_VERSION: OnceLock<String> = OnceLock::new();

    LIB_VERSION.get_or_init(|| {
        let version = env!("CARGO_PKG_VERSION");
        version.replace("-alpha", "a").replace("-beta", "b")
    })
}

#[pymodule]
fn _geohashr(py: Python, module: &Bound<PyModule>) -> PyResult<()> {
    module.add("__version__", get_lib_version())?;

    module.add_function(wrap_pyfunction!(decode, module)?)?;
    module.add_function(wrap_pyfunction!(decode_exact, module)?)?;
    module.add_function(wrap_pyfunction!(encode, module)?)?;
    module.add_function(wrap_pyfunction!(bbox, module)?)?;
    module.add_function(wrap_pyfunction!(neighbors, module)?)?;
    module.add_function(wrap_pyfunction!(neighbor, module)?)?;

    module.add("DecodeError", py.get_type_bound::<DecodeError>())?;
    module.add("EncodeError", py.get_type_bound::<EncodeError>())?;
    module.add("ParamError", py.get_type_bound::<ParamError>())?;

    Ok(())
}

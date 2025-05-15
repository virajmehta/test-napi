#![deny(clippy::all)]
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn get_user_from_env() -> String {
  let name: PyResult<String> = Python::with_gil(|py| {
    let locals = [("os", py.import("os")?)].into_py_dict(py)?;
    let code = c_str!("os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'");
    let user: String = py.eval(code, None, Some(&locals))?.extract()?;

    Ok(user)
  });

  name.unwrap()
}

#[napi]
pub fn grab_virajm_site() -> String {
  let body = reqwest::blocking::get("https://virajm.com")
    .unwrap()
    .text()
    .unwrap();
  body
}

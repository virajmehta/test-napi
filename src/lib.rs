#![deny(clippy::all)]
use std::time::Duration;

use anyhow::{bail, Result};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use reqwest::Client;

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

#[napi]
pub async fn grab_virajm_site_async() -> String {
  let client = Client::new();
  let body = client
    .get("https://virajm.com")
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
  body
}

#[napi(object)]
pub struct TimeAndNfl {
  pub time: String,
  pub team: String,
}

#[napi]
pub fn get_time_and_nfl() -> TimeAndNfl {
  let time = chrono::Utc::now().format("%H:%M:%S").to_string();
  let team = "Steelers".to_string();
  TimeAndNfl { time, team }
}

#[napi(object)]
pub struct Coefficients {
  pub a: f64,
  pub b: f64,
  pub c: f64,
}

#[napi]
pub enum SolutionType {
  OneRealRoot,
  TwoRealRoots,
}

#[napi(object)]
pub struct Solution {
  pub solution_type: SolutionType,
  pub root1: f64,
  pub root2: Option<f64>,
}

#[napi]
pub fn solve_quadratic(coefficients: Coefficients) -> Result<Solution> {
  let discriminant = coefficients.b * coefficients.b - 4.0 * coefficients.a * coefficients.c;
  println!("discriminant: {}", discriminant);
  if discriminant < 0.0 {
    bail!("No real roots");
  }
  if discriminant == 0.0 {
    return Ok(Solution {
      solution_type: SolutionType::OneRealRoot,
      root1: -coefficients.b / (2.0 * coefficients.a),
      root2: None,
    });
  }
  let root1 = (-coefficients.b + discriminant.sqrt()) / (2.0 * coefficients.a);
  let root2 = (-coefficients.b - discriminant.sqrt()) / (2.0 * coefficients.a);
  Ok(Solution {
    solution_type: SolutionType::TwoRealRoots,
    root1: root1,
    root2: Some(root2),
  })
}

#[napi]
pub async fn sleepy() -> String {
  tokio::time::sleep(Duration::from_secs(5)).await;
  "oh sorry just woke up!".to_string()
}

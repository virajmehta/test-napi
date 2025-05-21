#![deny(clippy::all)]
use std::collections::HashMap;
use std::sync::RwLock; // Using std::sync::RwLock for global state
use std::time::Duration;

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use napi::Status; // NAPI specific error status
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use reqwest::Client as HttpClient; // Renamed to avoid conflict with ClientExample
use uuid::Uuid;

#[macro_use]
extern crate napi_derive;

// --- Job Related Definitions (can stay top-level or be moved into a mod) ---

#[derive(Debug, PartialEq)]
#[napi]
pub enum JobStatusEnum {
  Pending,
  InProgress,
  Completed,
  Failed,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct JobUpdate {
  pub status: JobStatusEnum,
  pub progress: Option<u32>,
  pub result: Option<String>,
  pub error: Option<String>,
}

lazy_static! {
    // This remains global as tasks spawned by any client instance need to update it.
    static ref JOB_STATES: RwLock<HashMap<String, JobUpdate>> = RwLock::new(HashMap::new());
}

// --- Existing Code (trimmed for brevity, assuming it's the same as before) ---
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
  reqwest::blocking::get("https://virajm.com")
    .unwrap()
    .text()
    .unwrap()
}

#[napi]
pub async fn grab_virajm_site_async() -> String {
  let client = HttpClient::new(); // Use renamed HttpClient
  client
    .get("https://virajm.com")
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap()
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
    root1,
    root2: Some(root2),
  })
}

#[napi]
pub async fn sleepy() -> String {
  tokio::time::sleep(Duration::from_secs(5)).await;
  "oh sorry just woke up!".to_string()
}

// --- ClientExample with Integrated Job Methods ---
#[napi(js_name = "Client")]
pub struct ClientExample {
  pub env_value: String,
  client: HttpClient, // Use renamed HttpClient
}

#[napi]
impl ClientExample {
  #[napi(constructor)]
  pub fn new(env_name: String) -> Self {
    let env_value =
      std::env::var(env_name).unwrap_or_else(|_| "DEFAULT_VALUE_IF_NOT_SET".to_string());
    let client = HttpClient::new();
    ClientExample { env_value, client }
  }

  #[napi]
  pub fn get_value(&self) -> String {
    self.env_value.clone()
  }

  #[napi]
  pub async fn get_tensorzero_website(&self) -> String {
    self
      .client // Ensure this is using the HttpClient field
      .get("https://tensorzero.com")
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap()
  }

  #[napi]
  pub async fn start_long_job(&self) -> anyhow::Result<String> {
    // `&self` isn't strictly used here for the job logic itself as state is global,
    // but it makes sense for it to be a method of the client.
    // You could, for instance, log something with `self.env_value` if desired.
    // println!("Client with env_value '{}' is starting a job.", self.env_value);

    let job_id = Uuid::now_v7().to_string();
    let initial_status = JobUpdate {
      status: JobStatusEnum::Pending,
      progress: Some(0),
      result: None,
      error: None,
    };

    {
      let mut states = JOB_STATES.write().map_err(|e| {
        napi::Error::new(
          Status::GenericFailure,
          format!("Failed to acquire write lock on job states: {}", e),
        )
      })?;
      states.insert(job_id.clone(), initial_status);
    }

    let job_id_clone = job_id.clone();
    // The spawned task is independent of the `ClientExample` instance's lifetime after this point.
    tokio::spawn(async move {
      // Simulate some initial work
      tokio::time::sleep(Duration::from_secs(2)).await;
      {
        // In a real app, handle this unwrap better (e.g. log and skip update if poisoned)
        let mut states = JOB_STATES.write().unwrap();
        if let Some(job) = states.get_mut(&job_id_clone) {
          job.status = JobStatusEnum::InProgress;
          job.progress = Some(25);
        }
      }

      // Simulate more work
      tokio::time::sleep(Duration::from_secs(3)).await;
      {
        let mut states = JOB_STATES.write().unwrap();
        if let Some(job) = states.get_mut(&job_id_clone) {
          job.status = JobStatusEnum::InProgress;
          job.progress = Some(75);
        }
      }

      // Simulate completion
      tokio::time::sleep(Duration::from_secs(2)).await;
      {
        let mut states = JOB_STATES.write().unwrap();
        if let Some(job) = states.get_mut(&job_id_clone) {
          job.status = JobStatusEnum::Completed;
          job.progress = Some(100);
          job.result = Some("Long job finished successfully!".to_string());
        }
      }
    });

    Ok(job_id)
  }

  #[napi]
  pub fn poll_job_status(&self, job_id: String) -> Result<Option<JobUpdate>> {
    // `&self` is also not strictly necessary here for the current logic,
    // but good for consistency.
    // println!("Client with env_value '{}' is polling job: {}", self.env_value, job_id);

    let states = JOB_STATES.read().map_err(|e| {
      napi::Error::new(
        Status::GenericFailure,
        format!("Failed to acquire read lock on job states: {}", e),
      )
    })?;
    Ok(states.get(&job_id).cloned())
  }
}

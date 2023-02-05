use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::time::SystemTime;

static PROFILER: Lazy<Mutex<HashMap<String, Vec<f32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn profiler<'a>() -> MutexGuard<'a, HashMap<String, Vec<f32>>> {
    return PROFILER.lock().unwrap();
}

pub fn profiler_summary() -> HashMap<String, S> {
    let profiler = profiler();
    let mut summary = HashMap::new();
    for (k, v) in profiler.iter() {
        summary.insert(k.to_string(), S::new(v));
    }
    return summary;
}

pub fn print_profiler_summary() {
    let summary = profiler_summary();
    let mut keys: Vec<String> = summary.iter().map(|(k, _)| k.to_string()).collect();
    keys.sort();
    for key in keys.into_iter() {
        match summary.get(&key) {
            None => continue,
            Some(value) => {
                println!("{key}:\n  {value:?}");
            }
        }
    }
}

pub struct Profiler {
    start_time: SystemTime,
    key: String,
}

impl Profiler {
    pub fn start(key: &str) -> Self {
        let start_time = now();
        let key = key.to_string();
        return Self { start_time, key };
    }
    pub fn stop(&self) {
        let end_time = now();
        let time_since = end_time
            .duration_since(self.start_time)
            .unwrap()
            .as_secs_f32();

        let mut profiler = profiler();
        let times = profiler.entry(self.key.to_string()).or_insert(Vec::new());
        times.push(time_since);
    }
}

#[derive(Debug)]
pub struct S {
    pub num_calls: u64,
    pub total: f32,
    pub mean: f32,
}

impl S {
    pub fn new(times: &Vec<f32>) -> Self {
        let num_calls = times.len() as u64;
        let total: f32 = times.iter().sum();
        let mean = total / (num_calls as f32);
        return Self {
            num_calls,
            total,
            mean,
        };
    }
}

pub fn now() -> SystemTime {
    SystemTime::now()
}

pub fn since(start_time: SystemTime) -> f32 {
    return SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs_f32();
}

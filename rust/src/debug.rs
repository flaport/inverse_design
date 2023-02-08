use once_cell::sync::Lazy;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::time::SystemTime;

static _COUNTER: Lazy<Mutex<Counter>> = Lazy::new(|| Mutex::new(Counter { index: 0 }));
static _PROFILER: Lazy<Mutex<HashMap<String, Vec<f32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn profiler<'a>() -> MutexGuard<'a, HashMap<String, Vec<f32>>> {
    return _PROFILER.lock().unwrap();
}

pub fn counter<'a>() -> MutexGuard<'a, Counter> {
    return _COUNTER.lock().unwrap();
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
    let mut summary_vec: Vec<(&String, &S)> = summary.iter().collect();
    summary_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (key, value) in summary_vec.into_iter() {
        println!("{key}:\n  {value:?}");
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

impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        f32::partial_cmp(&self.total, &other.total).unwrap()
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        f32::partial_cmp(&self.total, &other.total)
    }
}

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

impl Eq for S {}

pub fn now() -> SystemTime {
    SystemTime::now()
}

pub fn since(start_time: SystemTime) -> f32 {
    return SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs_f32();
}

pub struct Counter {
    pub index: usize,
}

impl Counter {
    pub fn inc(&mut self) {
        self.index += 1;
    }
    pub fn dec(&mut self) {
        self.index -= 1;
    }
    pub fn eq(&self, value: usize) -> bool {
        self.index == value
    }
    pub fn gt(&self, value: usize) -> bool {
        self.index > value
    }
    pub fn value(&self) -> usize {
        self.index
    }
}

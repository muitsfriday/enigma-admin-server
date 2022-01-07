use std::error::Error;

pub mod mongo;

pub struct Experiment {
    id: Option<String>,
    title: String,
    description: String,
}

pub trait ExperimentRepo {
    fn save(expr: Experiment) -> Result<Experiment, Box<dyn Error>>;
}

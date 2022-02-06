pub mod experiment;
pub mod repo;

pub struct Experiment {
    id: Option<String>,
    title: String,
    description: String,
}

pub trait Repo {
    fn create(&self, data: Experiment) -> Result<Experiment, std::io::Error>;
}

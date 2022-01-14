pub struct Experiment {
    title: String,
    descrition: String,
}

pub trait Repo {
    fn create(&self, data: Experiment) -> Result<Experiment, std::io::Error>;
}


use mongodb::{Coll};

mod super::Experiment;

pub struct ExerimentRepo {
    coll: mongodb::Coll<>,
}

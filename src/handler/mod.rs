use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::service::experiment;

pub mod experiment_create;
pub mod experiment_get;
pub mod experiment_list;

/// API-wise struct
///
///
///

/// Experiment is a struct that contains the response of the create experiment endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experiment {
    id: Option<String>,
    title: String,
    description: String,
    active_interval: Option<Interval>,
    variations: Vec<Varience>,
    group_assign: experiment::GroupAssignment,
    owner: Option<HashMap<String, serde_json::Value>>,
    owner_group: String,
}

/// Interval is a tuple struct contains information abount datetime range.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

/// Varience is a struct contains data of each variance in experiment payload.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Varience {
    pub group: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

impl Into<experiment::Varience> for Varience {
    fn into(self) -> experiment::Varience {
        experiment::Varience {
            group: self.group.to_owned(),
            description: self.description.to_owned(),
            indicator: self.indicator.to_owned(),
            weight: self.weight.to_owned(),
            values: self.values.to_owned(),
        }
    }
}

/// GroupAssignment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupAssignment {
    pub strategy: String,
    pub persistent: String,
}

impl Into<experiment::GroupAssignment> for GroupAssignment {
    fn into(self) -> experiment::GroupAssignment {
        experiment::GroupAssignment {
            strategy: self.strategy.clone(),
            persistent: self.persistent.clone(),
        }
    }
}

/// Implementation for transforms request body to service's data model.
impl Into<experiment::Experiment> for Experiment {
    fn into(self) -> experiment::Experiment {
        let active_interval = match self.active_interval {
            Some(act) => Some(experiment::Interval(act.0, act.1)),
            None => None,
        };
        let vs: Vec<experiment::Varience> = self.variations.into_iter().map(|v| v.into()).collect();

        experiment::Experiment {
            id: None,
            title: self.title,
            description: self.description,
            active_interval,
            variations: vs,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            group_assign: self.group_assign.into(),
            owner: self.owner,
            owner_group: self.owner_group,
        }
    }
}

/// Implementation for transforms request body to service's data model.
impl From<experiment::Experiment> for Experiment {
    fn from(val: experiment::Experiment) -> Experiment {
        let active_interval = val.active_interval.map(|act| Interval(act.0, act.1));
        let vs = val
            .variations
            .iter()
            .map(|v| Varience {
                group: v.group.to_owned(),
                description: v.description.to_owned(),
                indicator: v.indicator.to_owned(),
                weight: v.weight.to_owned(),
                values: v.values.to_owned(),
            })
            .collect();

        Experiment {
            id: val.id.clone(),
            title: val.title,
            description: val.description,
            active_interval,
            variations: vs,
            group_assign: experiment::GroupAssignment {
                strategy: val.group_assign.strategy,
                persistent: val.group_assign.persistent,
            },
            owner: val.owner,
            owner_group: val.owner_group,
        }
    }
}

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use bson::Bson;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use mongodb::{bson::doc, bson::oid, Collection};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::service::experiment as service;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub _id: Option<oid::ObjectId>,
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub active_interval: Option<Interval>,

    pub variations: Vec<Variance>,
    pub classing: Classing,

    pub owner: Option<serde_json::Value>,
    pub channel_id: String,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variance {
    pub group_name: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Classing {
    pub strategy: String,
    pub persistent_mode: String,
}

impl From<service::Experiment> for Document {
    fn from(data: service::Experiment) -> Self {
        Self {
            _id: None,
            id: data.id,
            name: data.name,
            description: data.description,
            active_interval: data.active_interval.map(|v| v.into()),
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
            variations: data.variations.into_iter().map(|v| v.into()).collect(),
            classing: Classing {
                strategy: data.classing.strategy,
                persistent_mode: data.classing.persistent_mode,
            },
            owner: data.owner,
            channel_id: data.channel_id,
        }
    }
}

impl From<service::Interval> for Interval {
    fn from(data: service::Interval) -> Self {
        Self(data.0.clone(), data.1.clone())
    }
}

impl From<service::Variance> for Variance {
    fn from(data: service::Variance) -> Self {
        Self {
            group_name: data.group_name,
            description: data.description,
            indicator: data.indicator,
            weight: data.weight,
            values: data.values,
        }
    }
}

pub struct Repo {
    coll: Collection<Document>,
}

impl Repo {
    pub fn new(coll: Collection<Document>) -> Self {
        Self { coll }
    }
}

#[async_trait]
impl service::Store for Repo {
    async fn save(&self, data: &mut service::Experiment) -> Result<String> {
        let now = Utc::now();
        let mut document = Document::from(data.clone());

        document.updated_at = Some(now);
        if let None = document.created_at {
            document.created_at = Some(now);
        }

        if let None = document._id {
            document._id = Some(oid::ObjectId::new());
        }

        println!("repo insert: {:?}", document);
        let insert_result = self.coll.insert_one(document, None).await?;

        println!("repo insert_result");

        if let Bson::ObjectId(ref id) = insert_result.inserted_id {
            data.id = Some(id.to_hex());
        } else {
            data.id = None;
        }

        Ok(data.id.clone().unwrap_or_default())
    }
}

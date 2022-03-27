use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: HashMap<String, serde_json::Value>,
}

impl UserToken {
    pub fn get_user_group<'a>(&'a self) -> Option<&'a str> {
        return self.user.get("group_id").map(|v| v.as_str()).unwrap();
    }

    pub fn get_user_id<'a>(&'a self) -> Option<&'a str> {
        return self.user.get("id").map(|v| v.as_str()).unwrap();
    }
}

pub fn get_user_group(u: &HashMap<String, serde_json::Value>) -> String {
    let s = serde_json::Value::default();
    let h = u.get("group_id").unwrap_or(&s).as_str();
    return String::from(h.unwrap_or_default());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub alias: String,
    pub email: String,
}

impl UserToken {
    pub fn default() -> Self {
        UserToken {
            iat: 0,
            exp: 0,
            user: HashMap::default(),
        }
    }
}

impl User {
    pub fn default() -> Self {
        User {
            id: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            alias: "".to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: User,
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
            user: User::default(),
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

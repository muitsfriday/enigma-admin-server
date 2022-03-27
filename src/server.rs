#[derive(Clone)]
pub struct ServerConfig {
    pub url: String,

    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,

    pub jwt_secret: String,
}

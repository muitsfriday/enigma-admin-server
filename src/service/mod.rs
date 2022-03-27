pub mod experiment;
pub mod repo;

#[derive(Debug)]
pub struct Context<'a> {
    pub user_id: &'a str,
    pub user_group_id: &'a str,
}

#[derive(Debug)]
pub struct ConnOpts {
    pub host: String,
    pub password: Option<String>,
    pub database: String,
    pub port: u16,
    pub sql_impl: String,
    pub user: String,
}

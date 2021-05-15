fn default_pool_size() -> u8 {
    5
}

fn default_db_string() -> String {
    "postgres://postgres:password@localhost:5432/fts_test".to_owned()
}

fn default_file_path() -> String {
    "../simplewiki-20170820-pages-meta-current.xml".to_owned()
}

#[derive(Deserialize, Debug)]
pub struct Environment {
    #[serde(default = "default_db_string")]
    pub database_url: String,

    #[serde(default = "default_pool_size")]
    pub db_pool: u8,

    #[serde(default = "default_file_path")]
    pub file_path: String
}

pub fn get_env() -> Environment {
    match envy::from_env::<Environment>() {
        Ok(env) => env,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

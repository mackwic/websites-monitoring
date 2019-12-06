use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
pub struct Configuration {
    pub websites: Vec<String>,
    pub parallel_fetch: usize,
    pub interval_between_fetch: u64,
    pub stop_after_iteration: usize,
}

const CONFIG_PATH: &str = "./Configuration.toml";

pub fn load_configuration() -> Result<Configuration, String> {
    let path = Path::new(CONFIG_PATH);
    if !path.is_file() {
        return Err(format!("Configuration file is absent: {}", CONFIG_PATH));
    }

    let mut file = File::open(path).expect("Path should exists");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("reading file should be OK");
    let config_parsing_res = toml::from_str(&contents);

    config_parsing_res.or_else(|err| Err(format!("Unable to load configuration: {}", err)))
}

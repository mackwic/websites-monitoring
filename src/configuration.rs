use log::info;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
pub struct Configuration {
    #[serde(skip)]
    pub websites: Vec<String>,
    pub parallel_fetch: usize,
    pub interval_between_fetch: u64,
    pub stop_after_iteration: usize,
    pub run_forever: bool,
}

#[derive(Debug, serde::Deserialize)]
struct ConfigurationFromEnvironment {
    parallel_fetch: Option<usize>,
    interval_between_fetch: Option<u64>,
    stop_after_iteration: Option<usize>,
    run_forever: Option<bool>,
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
    let config_parsing_res = toml::from_str(&contents)
        .or_else(|err| Err(format!("Couldn't decode the Toml configuration: {}", err)));
    let default_config: Configuration = config_parsing_res?;

    let config_from_env: ConfigurationFromEnvironment = envy::prefixed("APP_")
        .from_env()
        .expect("Environment should be readable");

    // TODO handle error nicely
    let websites_from_db =
        crate::infrastructure::database::fetch_websites_to_crawl().expect("Should not have failed");

    let configuration = Configuration {
        interval_between_fetch: config_from_env
            .interval_between_fetch
            .unwrap_or(default_config.interval_between_fetch),
        parallel_fetch: config_from_env
            .parallel_fetch
            .unwrap_or(default_config.parallel_fetch),
        run_forever: config_from_env
            .run_forever
            .unwrap_or(default_config.run_forever),
        stop_after_iteration: config_from_env
            .stop_after_iteration
            .unwrap_or(default_config.stop_after_iteration),
        websites: websites_from_db,
    };
    info!("Configuration: {:#?}", configuration);
    Ok(configuration)
}

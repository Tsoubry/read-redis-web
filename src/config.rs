use clap::Parser;

const DEFAULT_HOST: &'static str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;

const DEFAULT_REDIS_HOST: &'static str = "127.0.0.1";
const DEFAULT_REDIS_PORT: u16 = 6379;
const DEFAULT_REDIS_USER: &'static str = "";
const DEFAULT_REDIS_PASSWORD: &'static str = "";
const DEFAULT_REDIS_DATABASE: &'static str = "";

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct AppConfig {
    #[arg(short, long, default_value = DEFAULT_HOST, help ="server host")]
    pub host: String,
    #[arg(short, long, default_value_t = DEFAULT_PORT, help ="server port")]
    pub port: u16,
    #[arg(long, default_value = DEFAULT_REDIS_HOST, help ="redis host")]
    pub redis_host: String,
    #[arg(long, default_value_t = DEFAULT_REDIS_PORT, help ="redis port")]
    pub redis_port: u16,
    #[arg(long, default_value = DEFAULT_REDIS_USER, help ="redis username, if required")]
    pub redis_user: String,
    #[arg(long, default_value = DEFAULT_REDIS_PASSWORD, help ="redis password, if required")]
    pub redis_pass: String,
    #[arg(long, default_value = DEFAULT_REDIS_DATABASE, help ="redis database, if required")]
    pub redis_db: String,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to json validate the redis output data (default false)"
    )]
    pub validate_json: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to add content-type application/json to the header, indicating the output is a json (default false)"
    )]
    pub output_json: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to decompress gzipped redis data first (default false)"
    )]
    pub decompress_gzip: bool,
}

impl AppConfig {
    pub fn get_optional_user_pass(&self) -> String {
        match (self.redis_user.as_str(), self.redis_pass.as_str()) {
            (user, pass) if user != "" && pass != "" => format!("{}:{}@", user, pass),
            (user, _) if user != "" => format!("{}@", user),
            _ => "".to_string(),
        }
    }

    pub fn get_redis_uri(&self) -> String {
        //The URL format is redis://[<username>][:<password>@]<hostname>[:port][/<db>]
        format!(
            "redis://{}{}:{}/{}",
            self.get_optional_user_pass(),
            self.redis_host,
            self.redis_port,
            self.redis_db
        )
    }
}

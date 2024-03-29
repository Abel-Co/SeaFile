use std::env;
use std::path::Path;
use std::sync::Arc;

use log4rs::config::RawConfig;
use once_cell::sync::OnceCell;
use log4rs::init_raw_config;

use crate::boot::conf::{Conf, Postgres};
use crate::module;

pub mod c;
pub mod conf;
pub mod middleware;

pub fn app_env<'a>() -> &'a Arc<String> {
    static ENV: OnceCell<Arc<String>> = OnceCell::new();
    ENV.get_or_init(|| {
        Arc::new(env::var("APP_ENV").unwrap_or("".to_owned()))  // unwrap_or
    })
}

pub fn get_config_path() -> String {
    if Path::new("./profiles").exists() {
        return format!("profiles/{}/", app_env());
    }
    return "./".to_string();
}

fn use_env(postgres: Option<Postgres>) -> Option<Postgres> {
    if let Ok(dsn) = env::var("DATABASE_DSN") {
        Some(Postgres { dsn, ..postgres.unwrap() })
    } else {
        postgres
    }
}

pub fn global<'a>() -> &'a Arc<Conf> {
    static CONFIG: OnceCell<Arc<Conf>> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let config_path = get_config_path();
        let s = std::fs::read_to_string(config_path.clone() + "config.yaml").unwrap();
        let mut conf: Conf = serde_yaml::from_str(&s).unwrap();
        conf.server.env = Some(app_env().as_str().to_string());
        conf.config_path = Some(config_path);
        conf.postgres = use_env(conf.postgres);
        Arc::new(conf)
    })
    // https://blog.csdn.net/u010766458/article/details/104579345
    // let appenv = std::fs::read_to_string("~/data/appenv").unwrap();
    // let env_vars: Vec<_> = appenv.lines().filter(|x| !x.is_empty()).collect();
    // println!("appenv: {:?}", env_vars);
    // let kvs = env_vars.into_iter().map(|x| x.split("=").collect::<Vec<_>>())
    //     .filter(|x| x.len() > 1).map(|x| (x[0], x[1])).collect::<HashMap<_, _>>();
    // println!(">>> var_map: {:?}", kvs);
    // println!(">>> env: {:?}", kvs.get("env2").unwrap_or(&"./"));
}

const CFG: &str = r#"
refresh_rate: 30 seconds
appenders:
  stdout:
    kind: console
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} {h({l})} [{M}] - {m}{n}"
root:
  level: trace
  appenders:
    - stdout
"#;

pub fn raw_config(level: &str) -> RawConfig {
    let log_cfg = CFG.replace("trace", level);
    ::serde_yaml::from_str::<RawConfig>(log_cfg.as_str()).unwrap()
}

pub async fn start() {
    // let config_path = global().config_path.clone().unwrap();
    // log4rs::init_file(config_path + "log4rs.yaml", Default::default()).unwrap();
    init_raw_config(raw_config("info")).unwrap();   // 1.初始化 日志
    c::init_rbatis().await;                         // 2.初始化 数据源
    c::init_db_schema().await;                      // 3.初始化 数据库
    module::start().await;                          // 4.业务模块 启动
}

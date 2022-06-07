use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug)]
struct Config {
    access_key: String,
    secret_access_key: String,
    session_token: String
}

impl Config {
    fn new(access_key:String, secret_access_key:String, session_token:String) -> Self {
        Config {
            access_key,
            secret_access_key,
            session_token,
        }
    }
}

fn main() {
    let new_config = get_config_from_clipboard();
    write_new_credentials_file(new_config)
}

fn get_config_from_clipboard() -> Config {
    let mut ctx = ClipboardContext::new().unwrap();
    let contents = ctx.get_contents().unwrap();
    let configs: Vec<&str> = contents.lines().collect();
    if configs.len() < 4 {
        panic!("Please copy your aws config to clipboard and try again");
    }

    let new_config = Config::new(String::from(configs[1]), String::from(configs[2]), String::from(configs[3]));
    println!("{}\n{}\n{}", new_config.access_key, new_config.secret_access_key, new_config.session_token);
    new_config
}

fn write_new_credentials_file(new_config: Config) {
    let path = Path::new("/home/sean/.aws/credentials");
    let mut write = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");
        
    write
        .write_fmt(format_args!("[default]\n{}\n{}\n{}", new_config.access_key, new_config.secret_access_key, new_config.session_token))
        .expect("unable to write data");
}

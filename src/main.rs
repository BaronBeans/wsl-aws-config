use std::{fs::{File, OpenOptions}, io::prelude::*};
use std::path::Path;

#[derive(Debug)]
struct Config {
    access_key: String,
    secret_key: String,
    security_token: String,
}

fn main() {
    let config = read_config_from_vault_file();
    write_new_credentials_file(config);
}

fn read_config_from_vault_file() -> Config {
    let mut access_key = String::from("aws_access_key_id=");
    let mut secret_key = String::from("aws_secret_access_key=");
    let mut security_token = String::from("aws_session_token=");

    let path = Path::new("/mnt/c/Users/sean.barnard/.aws/vault.txt");
    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    let split = contents.split("\n");
                    for line in split {
                        if line.starts_with("access_key") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            access_key.push_str(&parts[1]);
                            println!("{}", parts[1]);
                        };
                        if line.starts_with("secret_key") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            secret_key.push_str(&parts[1]);
                            println!("{}", parts[1]);
                        };
                        if line.starts_with("security_token") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            security_token.push_str(&parts[1]);
                            println!("{}", parts[1]);
                        };
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Config {
        access_key,
        secret_key,
        security_token,
    }
}

fn write_new_credentials_file(cfg: Config) {
    let path = Path::new("/home/sean/.aws/credentials");
    let mut write = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");

    write
        .write_fmt(format_args!(
            "[default]\n{}\n{}\n{}",
            cfg.access_key, cfg.secret_key, cfg.security_token
        ))
        .expect("unable to write data");
}

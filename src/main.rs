#![allow(dead_code)]
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{
    fs::{File, OpenOptions},
    io::prelude::*,
};

#[derive(Debug)]
struct Config {
    access_key: String,
    secret_key: String,
    security_token: String,
}

fn main() {
    get_vault_config();
    // let config = read_config_from_vault_file();
    // write_new_credentials_file(config);
}

fn get_vault_config() {
    println!("'dev' or 'qa'");
    let mut env_choice = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut env_choice).unwrap_or_default();

    Command::new("/mnt/c/Hashicorp/vault.exe")
        .arg("write")
        .arg("account/865412956762/sts/Owner")
        .arg("ttl=4h")
        .spawn()
        .expect("something went wrong with command 1")
        .wait()
        .expect("");

    // .arg("|")
    // .arg("Out-File")
    // .arg("-Encoding")
    // .arg("UTF8")
    // .arg("-FilePath")
    // .arg("/mnt/c/Users/barnars/.aws/vault.txt")
    // .spawn()
    // .expect("something went wrong running the binary")
    // .wait()
    // .expect("");
}

fn read_config_from_vault_file() -> Config {
    let mut access_key = String::from("aws_access_key_id=");
    let mut secret_key = String::from("aws_secret_access_key=");
    let mut security_token = String::from("aws_session_token=");

    let path = Path::new("/mnt/c/Users/barnars/.aws/vault.txt");
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

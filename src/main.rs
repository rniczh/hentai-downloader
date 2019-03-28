#[macro_use]
extern crate clap;
extern crate reqwest;

mod handler;
mod manga;
mod parser;

use crate::parser::Cli;

use clap::App;
use handler::Handler;
use manga::Manga;
use std::fs;
use std::path::Path;
use threadpool::ThreadPool;

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let Cli { url, cookie } = parser::parse_cli(&matches);

    let host = url.host().unwrap().to_string();

    let h = Handler::new(&host, &cookie);
    let m = Manga::new(&h, &url);

    println!("Collect Download information");
    let download_urls = m.get_download_urls(&h);

    // starting download
    let pool = ThreadPool::new(16);
    let path = format!("tmp{}", m.number);
    if !Path::new(&path).exists() {
        fs::create_dir(&path)?;
    }
    for target in download_urls {
        let path = path.clone();
        pool.execute(move || {
            // retry download 3 times if timeout
            // it will happen when the file is too slow in loading
            let times = 3;
            for _ in 0..times {
                match Handler::download(&target, &path) {
                    Ok(_) => break,
                    Err(ref e) if e.is_timeout() => {
                        println!("Download Timeout, retry download {}", target);
                    }
                    Err(ref e) => panic!(
                        "Something wrong when download file from url {}\nWith Error: {}",
                        target, e
                    ),
                }
            }
        });
    }

    pool.join();

    Ok(())
}

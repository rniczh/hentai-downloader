#[macro_use]
extern crate clap;
extern crate reqwest;

mod handler;
mod manga;

use clap::App;
use handler::Handler;
use manga::Manga;
use std::fs;
use std::path::Path;
use threadpool::ThreadPool;
use url::{Host, Url};

fn main() -> Result<(), Box<std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let url: String = matches
        .value_of("url")
        .expect("Should provide the url.")
        .parse::<String>()
        .expect("Incorrect url.");

    let download_url = Url::parse(url.trim())?;
    assert!(download_url.scheme() == "https");
    assert!(
        download_url.host() == Some(Host::Domain("e-hentai.org"))
            || download_url.host() == Some(Host::Domain("exhentai.org"))
    );

    let mut cookie = String::from("");
    if let Some(c) = matches.value_of("cookie") {
        if Path::new(&c).exists() {
            cookie = fs::read_to_string(&c)
                .expect("Something went wrong reading the cookie file")
                .trim()
                .to_string();
        }
    }

    let host = download_url.host().unwrap().to_string();

    let h = Handler::new(&host, &cookie);
    println!("Collect Download information");
    let m = Manga::new(&h, &download_url);
    let download_urls = m.get_download_urls(&h);

    // starting download
    let pool = ThreadPool::new(8);
    let path = format!("tmp{}", m.number);
    if !Path::new(&path).exists() {
        fs::create_dir(&path)?;
    }
    for target in download_urls {
        let f = path.clone();
        pool.execute(move || {
            Handler::download(&target, &f).expect("download failed");
        });
    }
    pool.join();

    Ok(())
}

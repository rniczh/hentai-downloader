use std::path::Path;
use url::{Host, Url};
use std::fs;

pub struct Cli {
    pub url: Url,
    pub cookie: String,
}

pub fn parse_cli(matches: &clap::ArgMatches) -> Cli {
    // parse url: String to download_url: url::Url
    let url: String = matches
        .value_of("url")
        .expect("Should provide the url.")
        .parse::<String>()
        .expect("Incorrect url.");

    let download_url: Url = Url::parse(url.trim()).expect("Parse url failed");
    assert!(download_url.scheme() == "https");
    assert!(
        download_url.host() == Some(Host::Domain("e-hentai.org"))
            || download_url.host() == Some(Host::Domain("exhentai.org"))
    );

    // read cookie file into cookie: String
    let mut cookie = String::from("");
    if let Some(c) = matches.value_of("cookie") {
        if Path::new(&c).exists() {
            cookie = fs::read_to_string(&c)
                .expect("Something went wrong reading the cookie file")
                .trim()
                .to_string();
        }
    }

    Cli {
        url: download_url,
        cookie: cookie,
    }
}

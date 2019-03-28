use reqwest::header::*;
use std::path::Path;

#[derive(Clone)]
pub struct Handler {
    pub client: reqwest::Client,
    pub host: String,
    cookie: String,
}

impl Handler {
    pub fn new(host: &str, cookie: &str) -> Self {
        Handler {
            client: reqwest::Client::new(),
            host: host.to_string(),
            cookie: cookie.to_string(),
        }
    }

    pub fn request(&self, task: &str, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let res = self
            .client
            .get(url)
            .header(COOKIE, &self.cookie[..])
            .header(HOST, &self.host[..])
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0",
            )
            .send();
        res
    }

    pub fn download(target: &str, path: &str) -> Result<(), reqwest::Error> {
        let dh = Handler::new("", "");
        let mut res = match dh.request("Download", &target) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let (mut dest, fname) = {
            let fname = res
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .expect("Extract file name failed.");

            let fname = Path::new(path).join(fname);
            (
                std::fs::File::create(&fname).expect(&format!(
                    "Create file failed with file: {}",
                    fname.to_str().unwrap()
                )),
                fname
            )
        };
        println!("Downloading: {}", fname.to_str().unwrap());
        if let Err(_) = std::io::copy(&mut res, &mut dest) {
            println!("File download failed: {} ", fname.to_str().unwrap());
        }
        Ok(())
    }
}

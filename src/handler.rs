use reqwest::header::*;

pub struct Handler {
    client: reqwest::Client,
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

    pub fn request(&self, task: &str, url: &str) -> reqwest::Response {
        let res = self
            .client
            .get(url)
            .header(COOKIE, &self.cookie[..])
            .header(HOST, &self.host[..])
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0",
            )
            .send()
            .expect(&format!("{}: Request failed with path: {}", task, url));
        res
    }

    pub fn download(target: &str, path: &str) -> Result<(), Box<std::error::Error>>  {
        let dh = Handler::new("", "");
        let mut res = dh.request("download", &target);
        let (mut dest, fname) = {
            let fname = res
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .expect("can't download");
            let fname = &format!("{}/{}", path, fname);
            (std::fs::File::create(fname).expect("Create file failed."), fname.clone())
        };
        println!("Downloading: {}", fname);
        if let Err(_) = std::io::copy(&mut res, &mut dest) {
            println!("File download failed: {} ", fname);
        }
        Ok(())
    }
}

extern crate reqwest;
use crate::handler::Handler;
use core::cmp::max;
use select::document::Document;
use select::predicate::Name;

pub struct Manga {
    pub number: u32,
    pub pages: u32,
    pub url: reqwest::Url
}

impl Manga {
    pub fn new(h: &Handler, url: &reqwest::Url) -> Self {
        Manga {
            number: Self::get_hentai_number(url),
            pages: Self::get_page_number(h, url),
            url: url.clone(),
        }
    }
    fn get_hentai_number(url: &reqwest::Url) -> u32 {
        let segs = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

        segs[1].parse::<u32>().expect("Incorrect url")
    }
    fn get_page_number(h: &Handler, url: &reqwest::Url) -> u32 {
        let mut pages = 0;
        let res = h.request("Get Page number", &url[..]);
        Document::from_read(res)
            .expect("Document read response failed.")
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| {
                if x.contains("?p=") {
                    let num = x.split("?p=").last();
                    pages = max(num.unwrap().parse::<u32>().unwrap(), pages);
                }
            });
        pages
    }

    pub fn get_download_urls(&self, h: &Handler) -> Vec<String> {
        let tmp = match &(h.host.to_string())[..] {
            "exhentai.org" => "exhentai",
            "e-hentai.org" => "ehgt",
            _ => panic!("shound not happend"),
        };

        let download_url = &self.url;
        let mut download_urls: Vec<String> = vec![];
        for i in 0..self.pages + 1 {
            let res = h.request(
                "Get each Page",
                &(download_url.to_string() + &format!("?p={}", i)),
            );
            Document::from_read(res)
                .expect("Document read response failed.")
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| {
                    if x.contains("s/") {
                        let res = h.request("get page", x);
                        Document::from_read(res)
                            .unwrap()
                            .find(Name("img"))
                            .filter_map(|n| n.attr("src"))
                            .for_each(|x| {
                                if !x.contains(tmp) {
                                    download_urls.push(x.to_string());
                                    println!("{}", x);
                                }
                            });
                    }
                });
        }
        download_urls
    }
}

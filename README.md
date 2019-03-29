# E(X)Hentai Downloader

**Download Manga from e-hentai/exhentai**

Be careful, the E(x)Hentai website has image limit for user.

#![limit](https://i.imgur.com/OglhmK5.png)

So, you will take hours to recover if you exceed this image limit.

## Installation

See the Release page

Support `Linux`, `Windows` right now.

## Compilation

Make sure you have installed `rustup`, link: https://rustup.rs/

and input this command:

```
cargo +nightly build --release
```

Then the exectable will placed in target/release/hentai-downloader

## Usage

See `hentai-downloader --help` for help.

```
hentai-downloader 0.1
H.-S Zheng <mathan0203@gmail.com>
Download the Manga from e(x)hentai website.

USAGE:
    hentai-downloader [OPTIONS] --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cookie <file>    The cookie file for access exhentai.org
    -u, --url <url>        The url of Manga for which you want to downloa

```


**Example.**

`hentai-downloader -u https://e-hentai.org/g/12345/hijklmn/ `

And it will download the Manga into tmp12345 directory

If you are exhentai user then you can provide you cookie file with `-c ` option

`hentai-downloader -u https://exhentai.org/g/12345/hijklmn/ -c cookie.txt`

## Cookies

How to get the cookie.txt ?

Copy this information (take FireFox as example) to cookie.txt

![example](https://i.imgur.com/kUBPTyn.png)

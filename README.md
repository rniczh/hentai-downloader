# hentai-download

Download Manga from e-hentai/exhentai

## Installation

See the Release page

Only support Lniux right now.

I will provide the windows or mac version if I am free.
(also i'm not sure how to cross-compile it)

It may be easy for you to compile with Cargo if you in a hurry.


## Compilation

```
cargo build --release
```

Then the exectable will placed in target/release/hentai-downloader

## Usage

See `hentai-downloader --help` for help.

Example.

`hentai-downloader -u https://e-hentai.org/g/abcdefg/hijklmn/ `

And it will download the Manga into tmp12345 directory

If you are exhentai user then you can provide you cookie file with `-c ` option

`hentai-downloader -u https://exhentai.org/g/abcdefg/hijklmn/ -c cookie.txt`

## Cookies

How to get the cookie.txt ?

Copy this information (take FireFox as example) to cookie.txt

![example](https://i.imgur.com/kUBPTyn.png)

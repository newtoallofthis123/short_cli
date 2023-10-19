# NoobShort CLI (nshrt)

> version 0.2

NoobShort is a simple to use URL shortener. It is just a small part of
my site [noobscience.rocks](https://noobscience.rocks).
I made a very pretty web interface for it, but I also wanted to make a CLI
version of it. So here it is.

You can check out the web interface at [noobscience/go](https://noobscience.rocks/go).

## Screen shot

![Screenshot](/assets/screenshot.png)

## Installation

~~You need to manually compile it~~

Cargo binary is now available. You can install it with cargo by running

```bash
cargo install nshrt
```

### Local compilation

You are free to compile it yourself. You need to have Rust >= 1.4.0 and Cargo >= 0.6.0.
Then, just clone the repo and run `cargo build --release`. The binary will
be in `target/release/nshrt`.

```bash
git clone https://github.com/newtoallofthis123/short_cli/
cd short_cli
cargo build --release
sudo cp target/release/nshrt /usr/local/bin/nshrt
```

However, if you are not a rustecean, you can download the binary from
the [releases](https://github.com/newtoallofthis123/short_cli/releases)
page. Just download the binary for your OS and architecture and put it
somewhere in your PATH.

> I need your help. I don't have a Mac, so I can't compile the binary for
Mac. If you have a Mac, please compile the binary and send it to me.
Thanks!

## Usage

NoobShort CLI is very easy to use. Just type `nshrt <url>` and it will
return the shortened URL. If you want to use a custom URL, type
`nshrt <url> --custom <custom>`. If the custom URL is already taken, it will
return an error.

It is very simple and easy to use. I hope you enjoy it.

Just a small thing though, I am sorry if the interface has too much
branding. I am not trying to advertise, I just want to make sure that
people know where the URL is coming from. I hope you understand.

## Dev Stuff

NoobShort CLI is written primarily in Rust 1.7.0. It uses multiple
dependencies, which are listed in the Cargo.toml file.

It posts a request to the publicly available API at
`https://noobscience.rocks/api/go`. The API is written in TypeScript and
uses MongoDB as a database.
It is publicly available and you are free to integrate it into your own
projects. Just make sure to give me credit and keep in mind the [Terms of Service](https://noobscience.rocks/tos).

If you want to contribute, feel free to fork the repo and make a pull
request. I will review it and merge it if it is good.

To develop, open it in your favorite editor and start coding. To run it,
just run `cargo run` and it will compile and run it for you.

## License

NoobShort CLI is licensed under the MIT license. You can find it in the
[LICENSE](LICENSE) file.

## Contact

If you have any questions, feel free to contact me at
[noobscience/contact](https://noobscience.rocks/contact).

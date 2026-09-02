## What is a short?

This is the CLI for the [url-shortener](https://github.com/maksimshchavelev/url-shortener) 
project. This command-line utility connects to a remote shortener server, 
requests that it generate a short link, and returns it.

## Usage

> To generate a short link, you must first specify the shortener server using 
> the `SHORTENER_SERVER` environment variable

```
short https://example.com
```

You can specify the server manually (A short link will be generated for https://example.com):

```
short --server https://example.org https://example.com
```

## Building

You can build the project using cargo and rustc version `1.96.1`:

```
cargo build release
```

## Installation

Use this command to install `short`:

```
cargo install --path .
```

Executable will be installed to `~/.cargo/bin/`

## License

This project is licensed under the [MIT License](LICENSE)

# What is a short?

This is the CLI for the [url-shortener](https://github.com/maksimshchavelev/url-shortener) 
project. This command-line utility connects to a remote shortener server, 
requests that it generate a short link, and returns it.

# Usage

To generate a short link, you must first specify the shortener server using 
the `SHORTENER_SERVER` environment variable

You can specify the server manually (A short link will be generated 
for https://example.com):

```
short --server https://example.org https://example.com
```

## Generate a short link with default lifetime and without clicks limit

```
short https://example.com
```

> Lifetime depends on server's settings

## Generate a short link with specific lifetime and clicks limit

```
short --lifetime 1d --clicks 10 https://example.com
```

Generated short link will have 1 day lifetime and a click limit of 10.

## Discover a short link

You can discover a short link to get info about it (count of clicks,
clicks limit, date of creation and expiration date) by command:

```
short discover <SHORT LINK>
```

`SHORT LINK` can be a short code or a full link. For example, `SHORT LINK` can be
`https://example.com/Xd6ewWKe` or just `Xd6ewWKe`

Possible output:

```
Original URL:    https://example.com/
Count of clicks: 0
Clicks limit:    10
Created at:      05.09.2026 15:14:26
Expires at:      06.09.2026 15:14:26
```

# Building

You can build the project using cargo and rustc version `1.96.1`:

```
cargo build release
```

# Installation

Use this command to install `short`:

```
cargo install --path .
```

Executable will be installed to `~/.cargo/bin/`

# License

This project is licensed under the [MIT License](LICENSE)

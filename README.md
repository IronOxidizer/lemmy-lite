# lemmy-lite
A static, nojs, touch-friendly Lemmy frontent built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs)
- [Maud](https://maud.lambda.xyz)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get lite version of any Lemmy instance.
- NoJS using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer 11 compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> request per page.
  - Tiny, highly compressed pages.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- `Cargo +nightly run --release`

## Footnotes

1. First page load fetches a CSS stylesheet, favicon, and logo. These are cached so all subsequent pages require only a single HTML request.

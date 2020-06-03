# lemmy-lite
A static, nojs, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy

***NOTE:*** This project uses Maud-git(master) as it depends on actix 2.0.0. The necessary features are in Maud's master branch but are not yet in any official Maud release. As such, this project is very unstable and is subject to breaking / major changes.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs)
- [Maud](https://maud.lambda.xyz)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- NoJS using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer 11, NetSurf, and Dillo compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> request per page.
  - Tiny, highly compressed pages.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- `Cargo +nightly run --release`

## Footnotes

1. First page load fetches a CSS stylesheet, favicon, and logo. These are cached so all subsequent pages require only a single HTML request.

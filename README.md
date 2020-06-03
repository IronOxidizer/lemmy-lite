# lemmy-lite
A static, nojs, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy

***NOTE:*** Putting this project on pause until Maud has a new release with actix 2.0.0 and async / await support as it will result in breaking changes that I want to avoid re-writing. These changes are already in the master branch, however there has not been a new release that includes it. I expect this release will come out in the upcoming weeks. I might try to work around it by using maud-git(master) however it's unlikely I'll push this work as it would be completely unstable.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs)
- [Maud](https://maud.lambda.xyz)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
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

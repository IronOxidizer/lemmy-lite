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
- Static, only one request per page.
- NoJS using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer 11 compatible.
- High performance.
  - Written in rust.
  - Tiny, highly compressed pages.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- `Cargo run --release`

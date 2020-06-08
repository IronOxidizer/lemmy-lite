# lemmy-lite
A static, nojs, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy

***NOTE:*** This project uses Maud-git(master) as it depends on actix 2.0.0. The necessary features are in Maud's master branch but are not yet in any official Maud release. Maud also uses Rust nightly. As such, this project is very unstable and is subject to breaking / major changes.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs)
- [Maud](https://maud.lambda.xyz)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- NoJS using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer and NetSurf compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> tiny request per page.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- `cp -r /home/main/Development/lemmy-lite/static /etc/nginx/` (Symlink won't always work if it isn't root)
- `ln -s /home/main/Development/lemmy-lite/lemmy-lite.conf /etc/nginx/sites-enabled/`
- `systemctl start nginx && cargo +nightly run --release`

## Footnotes

1. First load fetches a CSS stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
2. I use CSS Tables because it's faster and simpler than FlexBox and because Grid is broken on IE11 and NetSurf. Using CSS tables over HTML tables avoids excess DOM objects.
3. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
4. Each static file contains an uncompressed equivalent in /uncompressed.
5. 1.0.0 will is set for when account functionality is stabilized.
6. Static files will have shortened names, the originals in /uncompressed will have full name.
7. Create macro depending on release or debug build to use compressed on uncompressed file names.
8. Use 1 letter HTML class names.
9. Consider not supporting UTF-8 and only using ASCII charcters for better legacy font support.
10. Try combining HTML meta tags.
11. Change MAUD to create smaller HTML (single, self closing tags).
12. Move /etc/nginx/static to /etc/nginx/lemmy-lite/static
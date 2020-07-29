# lemmy-lite
A static, JSless, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy. Eventually it will transition into a microservice that is ran alongside Lemmy, for example, under a *lite.lemmy.com* sub domain. Ideally it will run on the same machine removing any extra latency in API calls.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs) - [Benchmarks](https://www.techempower.com/benchmarks/#test=composite)
- [Maud](https://maud.lambda.xyz) - [Benchmarks](https://ironoxidizer.github.io/ironoxidizer/blog/20200623-fastest-templating-engine)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- JSless using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer and NetSurf compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> tiny request per page.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- Symlink won't always work since nginx user (root) requires ownership of linked file
- GZip static to allow serving of compressed files for lower bandwith usage
```
cd lemmy-lite
gzip -kr9 static
cp -rf static /etc/nginx/lemmy-lite
ln -sf lemmy-lite.conf /etc/nginx/sites-enabled/
systemctl start nginx
cargo +nightly run --release
```

## Pictures

lemmy-lite: FireFox
![lemmy-lite: FireFox](https://user-images.githubusercontent.com/60191958/84398555-1872a280-abce-11ea-8e87-a06b3165a77e.png)

lemmy-lite: Mobile
![lemmy-lite: Mobile](https://user-images.githubusercontent.com/60191958/84398664-39d38e80-abce-11ea-862d-d2d5cb98a89b.png)

## Footnotes

1. First load fetches a stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
2. I use CSS Tables because it's [faster](https://benfrain.com/css-performance-test-flexbox-v-css-table-fight) and simpler than FlexBox, and because Grid is broken on IE11 and NetSurf. Using CSS tables over HTML tables avoids excess DOM objects.
3. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
4. 1.0.0 is set for when account functionality is stabilized.
5. Ideally, static content is served through a CDN to further improve server performance and response times.

## TODO

1. Implement pagination and sort UI. Try putting sort in same form as page to send multiple params.
2. Implement post search backend.
3. Impelemnt post search UI.
4. Refactor Navbar based on new UI.
5. Implement server side markdown rendering for comments and text post body using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).
6. Add instance info column that moves to the bottom when there's no space like `Lemmy` in mobile view.
7. Add URL handling for `/u/username/view/[overview/comments/posts/saved]`.
8. Use 1 letter HTML class names.
9. Use 1 letter static file names except for favicon and index.
10. Consider not supporting UTF-8 and only using ASCII charcters for data size and better legacy font support.
11. Consider switching from Maud to [Sailfish](https://github.com/Kogia-sima/sailfish/tree/master/benches) to improve performance.

## NetSurf Quirks

1. CSS `checked` is not implemented for `input[checkbox]` causing comment thread collapse to not work
2. CSS `word-spacing` is not implemented

## iOS Quirks

1. Highlights break with non-default CSS `word-spacing`

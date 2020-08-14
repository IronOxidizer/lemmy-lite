# lemmy-lite
A static, JSless, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering [Lemmy](https://github.com/LemmyNet/lemmy). Eventually it will transition into a microservice that is ran alongside [Lemmy](https://github.com/LemmyNet/lemmy), for example, under a *lite.lemmy.com* sub domain. Ideally it will run on the same machine removing any extra latency in API calls.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs) - [Benchmarks](https://www.techempower.com/benchmarks/#test=composite)
- [Maud](https://maud.lambda.xyz) - [Benchmarks](https://ironoxidizer.github.io/ironoxidizer/blog/20200623-fastest-templating-engine)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - [Benchmarks](https://github.com/IronOxidizer/markdown-benchmarks)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- JSless using pre-rendered HTML and CSS only.
- Touch-friendly.
- Small screen support, as small as 320px.
- Internet Exporer and NetSurf compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> tiny request per page.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- Symlink won't work since nginx user (root) requires ownership of linked file
- GZip static to allow serving of compressed files for lower bandwith usage
- Rust nightly is required, use `rustup`
```
cd lemmy-lite
gzip -kr9 static
cp -rf static /etc/nginx/lemmy-lite
cp -f lemmy-lite.conf /etc/nginx/sites-enabled/
systemctl start nginx
cargo +nightly run --release
```

## Pictures

Desktop|Mobile
---|---
![Desktop](https://user-images.githubusercontent.com/60191958/90257112-b542fd80-de14-11ea-9b84-752b5b691631.png)|![Mobile](https://user-images.githubusercontent.com/60191958/90256779-351c9800-de14-11ea-8189-092283c8fe28.png)

## Notes

1. As of 0.2.0 (Aug2020) worst case scenario ([240 comment thread](https://lemmylite.crabdance.com/dev.lemmy.ml/post/30493)) takes 9ms to render on server.
2. First load fetches a stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
3. I use CSS Tables instead of FlexBox and Grid because Tables are [faster](https://benfrain.com/css-performance-test-flexbox-v-css-table-fight), simpler, and have much better legacy support. Using CSS tables over HTML tables avoids excess DOM objects.
4. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
5. 1.0.0 is set for when account functionality is stabilized.
6. Ideally, static content is served through a CDN to further improve server performance and response times.
7. Strictly only uses characters from [BMP](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) for legacy font support.
8. Catch me developing lemmy-lite on my streams at [Twitch](https://www.twitch.tv/ironoxidizer) or [YouTube](https://www.youtube.com/channel/UCXeNgKTWtqOuIMXnhBHAskw)

## TODO

1. Fix `rustls` panic.
2. Add URL handling for `/u/username/view/[overview/comments/posts/saved]`.
3. Use 1 letter HTML class names.
4. Use 1 letter static file names except for favicon and index.
5. Consider not supporting UTF-8 and only using ASCII charcters for data size and better legacy font support.
6. Consider switching from Maud to [Sailfish](https://github.com/Kogia-sima/sailfish/tree/master/benches) to improve performance.

## NetSurf Quirks

1. CSS `checked` is not implemented for `input[checkbox]` causing comment thread collapse to not work
2. CSS `word-spacing` is not implemented

## iOS Quirks

1. Highlights break with non-default CSS `word-spacing`

## Update Script

```
killall lemmy-lite
git pull
gzip -rk9f static
sudo rm -rf /etc/nginx/lemmy-lite
sudo cp -rf static /etc/nginx/lemmy-lite
nohup cargo +nightly run --release &
```

# lemmy-lite
A static, JSless, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering [Lemmy](https://github.com/LemmyNet/lemmy). Eventually it will transition into a microservice that is ran alongside [Lemmy](https://github.com/LemmyNet/lemmy), for example, under a *lite.lemmy.com* sub domain. Ideally it will run on the same machine removing any extra latency in API calls.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs) - [Benchmarks](https://www.techempower.com/benchmarks/#test=composite)
- [Maud](https://maud.lambda.xyz) - [Benchmarks](https://ironoxidizer.github.io/ironoxidizer/blog/20200623-fastest-templating-engine)

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

lemmy-lite: FireFox
![lemmy-lite: FireFox](https://user-images.githubusercontent.com/60191958/84398555-1872a280-abce-11ea-8e87-a06b3165a77e.png)

lemmy-lite: Mobile
![lemmy-lite: Mobile](https://user-images.githubusercontent.com/60191958/84398664-39d38e80-abce-11ea-862d-d2d5cb98a89b.png)

## Notes

1. First load fetches a stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
2. I use CSS Tables instead of FlexBox and Grid because Tables are [faster](https://benfrain.com/css-performance-test-flexbox-v-css-table-fight), simpler, and have much better legacy support. Using CSS tables over HTML tables avoids excess DOM objects.
3. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
4. 1.0.0 is set for when account functionality is stabilized.
5. Ideally, static content is served through a CDN to further improve server performance and response times.
6. Strictly only uses characters from [BMP](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) for legacy font support.
7. Catch me developing lemmy-lite on my streams at [Twitch](https://www.twitch.tv/ironoxidizer) or [YouTube](https://www.youtube.com/channel/UCXeNgKTWtqOuIMXnhBHAskw)

## TODO

1. Fix communities, make markup modular to be reused in search.
2. Implement post search page and UI including filters.
3. If within a community, restrict search to community by default.
4. Implement server side markdown rendering for comments and text post body using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).
5. Add instance info column that moves to the bottom when there's no space like `Lemmy` in mobile view.
6. Add URL handling for `/u/username/view/[overview/comments/posts/saved]`.
7. Use 1 letter HTML class names.
8. Use 1 letter static file names except for favicon and index.
9. Consider not supporting UTF-8 and only using ASCII charcters for data size and better legacy font support.
10. Consider switching from Maud to [Sailfish](https://github.com/Kogia-sima/sailfish/tree/master/benches) to improve performance.

## NetSurf Quirks

1. CSS `checked` is not implemented for `input[checkbox]` causing comment thread collapse to not work
2. CSS `word-spacing` is not implemented

## iOS Quirks

1. Highlights break with non-default CSS `word-spacing`

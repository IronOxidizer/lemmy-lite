# lemmy-lite
A static, JSless, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering [Lemmy](https://github.com/LemmyNet/lemmy). Eventually it will transition into a microservice that is ran alongside Lemmy, for example, under *lite.lemmy.com* or *lemmy.com/lite*. Ideally it will run on the same machine removing any extra latency in API calls.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs) - [Benchmarks](https://www.techempower.com/benchmarks/#test=composite)
- [Maud](https://maud.lambda.xyz) - [Benchmarks](https://ironoxidizer.github.io/ironoxidizer/blog/20200623-fastest-template-engine)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - [Benchmarks](https://github.com/1Password/markdown-benchmarks)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- JSless using pre-rendered HTML and CSS only.
- Touch and mobile friendly.
- Small screen support, as small as 320px.
- Internet Exporer and NetSurf compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> tiny request per page.
  - Supports arm64 / Raspberry Pi.
  
## Installation

- Symlinks won't work since nginx user (root) requires ownership of linked file
- Pigz static to allow serving of compressed files for lower bandwidth usage
```
cd lemmy-lite
pigz -rk11f static
cp -rf static /etc/nginx/lemmy-lite
cp -f lemmy-lite.conf /etc/nginx/sites-enabled/
systemctl start nginx
cargo run --release
```

## Pictures

Android|Desktop|iOS
---|---|---
![android](https://user-images.githubusercontent.com/60191958/96905327-4dd5a980-1466-11eb-973c-476ae3af27e5.png)|![desktop](https://user-images.githubusercontent.com/60191958/96905850-fedc4400-1466-11eb-8902-f8aea874b670.png)|![ios](https://user-images.githubusercontent.com/60191958/96905906-11ef1400-1467-11eb-8f56-f4f8b336a3c5.png)

## Notes

1. As of 0.2.0 (Aug2020) worst case scenario ([240 comment thread](https://lemmylite.crabdance.com/dev.lemmy.ml/post/30493)) takes 9ms to render on server.
2. First load fetches a stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
3. I use CSS Tables instead of FlexBox and Grid because Tables are [faster](https://benfrain.com/css-performance-test-flexbox-v-css-table-fight), simpler, and have much better legacy support. Using CSS tables over HTML tables avoids excess DOM objects.
4. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
5. 1.0.0 is set for when account functionality is stabilized.
6. Ideally, static content is served through a CDN to further improve server performance and response times.
7. Strictly only uses characters from [BMP](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane) for legacy font support.
8. Catch me developing lemmy-lite on my streams at [Twitch](https://www.twitch.tv/ironoxidizer) or [YouTube](https://www.youtube.com/channel/UCXeNgKTWtqOuIMXnhBHAskw)
9. SVGs hand optimized using [Aydos SVG Path Editor](https://aydos.com/svgedit/)

## TODO

1. Fix user and community links (change markdown interpretation for same-site links)
2. Consider not supporting UTF-8 and only using ASCII characters for data size and better legacy font support.
3. Consider switching from Maud to [Sailfish](https://github.com/Kogia-sima/sailfish/tree/master/benches) to improve performance.

**Quirks**

1. CSS `word-spacing` is broken on iOS and NetSurf

## Update Script

```
killall lemmy-lite
git pull
rm static/*.gz
pigz -rk11f static
for i in static/*gz; do
  [ -f "$i" ] || break
  j="${i%.*}"
  if((`stat -c%s "$i"`>=`stat -c%s "$j"`));then
    echo "$i is larger than base, deleting"
    rm -f "$i"
  fi
done
sudo rm -rf /etc/nginx/lemmy-lite
sudo cp -rf static /etc/nginx/lemmy-lite
nohup cargo run --release &
```

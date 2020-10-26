FROM rust:alpine as builder
WORKDIR /usr/src/lemmy-lite
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path .
RUN strip /usr/local/cargo/bin/lemmy-lite

FROM alpine
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/lemmy-lite .
COPY static .
CMD ["/app/lemmy-lite"]
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM clux/muslrust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/stayfolio-bot

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/stayfolio-bot*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 stayfolio-bot

RUN adduser -D -s /bin/sh -u 1000 -G stayfolio-bot stayfolio-bot

WORKDIR /home/stayfolio-bot/bin/

COPY --from=cargo-build /usr/src/stayfolio-bot/target/x86_64-unknown-linux-musl/release/stayfolio-bot .

RUN chown stayfolio-bot:stayfolio-bot stayfolio-bot

USER stayfolio-bot

CMD ["./stayfolio-bot"]


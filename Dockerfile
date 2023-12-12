# Rust 1.74.1 is latest as of Dec 12, 2023
FROM rust:1.74.1 as build

RUN rustup target add $(arch)-unknown-linux-musl $(arch)-unknown-linux-gnu && \
    apt-get update && apt-get install -y musl-tools

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall cross

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Create a fake binary target to be used for dependency caching locally, then clean it
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cross build \
    && rm src/main.rs

COPY src ./src

RUN touch -am src/main.rs \
    && cargo test \
    && mkdir ./bin

# Use a statically linked target for the prod
RUN cross build --release --target $(arch)-unknown-linux-musl \
    && mv ./target/$(arch)-unknown-linux-musl/release/httpget ./bin/httpget

RUN cross build --release --features tls --target $(arch)-unknown-linux-musl \
    && mv ./target/$(arch)-unknown-linux-musl/release/httpget ./bin/httpget-tls


FROM scratch as runner

COPY --from=build /app/bin/httpget /

ENTRYPOINT ["/httpget"]


FROM scratch as runner-tls

COPY --from=build /app/bin/httpget-tls /httpget

ENTRYPOINT ["/httpget"]
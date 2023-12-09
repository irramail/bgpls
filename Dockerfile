FROM rust:1.74.1-bookworm as builder
WORKDIR /app/src
COPY ./Cargo* .
RUN --mount=type=cache,target=/usr/local/cargo/registry mkdir "src" && echo "fn main() {}" > "src/main.rs" && cargo build --release
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

FROM debian:12.2 as server
COPY --from=builder /app/src/target/release/bgpls /usr/local/bin
EXPOSE 3032
ENTRYPOINT [ "/usr/local/bin/bgpls" ]

FROM debian:12.2 as binaries
COPY --from=builder /app/src/target/release/bgpls /
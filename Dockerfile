FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/myapp
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /home/myapp/bin/
COPY --from=cargo-build /usr/src/myapp/target/x86_64-unknown-linux-musl/release/udpdns .
CMD ["udpdns"]

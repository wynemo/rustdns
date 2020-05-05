FROM rust:latest as cargo-build
#RUN apt-get update
#RUN apt-get install musl-tools gcc-arm-linux-gnueabi make git-core ncurses-dev -y
#RUN rustup target add x86_64-unknown-linux-musl
#RUN rustup target add armv7-unknown-linux-gnueabihf
WORKDIR /usr/src/myapp
COPY . .
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
#RUN RUSTFLAGS=-Clinker=arm-none-linux-gnueabihf-gcc cargo build --release --target=armv7-unknown-linux-gnueabihf
#RUN cargo build --release --target=armv7-unknown-linux-musleabihf
RUN cargo build --release

FROM alpine:latest
WORKDIR /home/myapp/bin/
#COPY --from=cargo-build /usr/src/myapp/target/x86_64-unknown-linux-musl/release/udpdns .
COPY --from=cargo-build /usr/src/myapp/target/release/udpdns .
CMD ["udpdns"]

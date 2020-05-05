FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add my_target
WORKDIR /usr/src/myapp
COPY . .
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
#RUN RUSTFLAGS=-Clinker=arm-none-linux-gnueabihf-gcc cargo build --release --target=armv7-unknown-linux-gnueabihf
RUN /bin/ash -c 'set -ex && \
    ARCH=`uname -m` && \
    if [ "$ARCH" == "x86_64" ]; then \
       echo "x86_64" && \
       RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=my_target; \
    else \
       echo $ARCH && \
       RUSTFLAGS=-Clinker=arm-none-linux-gnueabihf-gcc cargo build --release --target=my_target; \
    fi'

FROM alpine:latest
WORKDIR /home/myapp/bin/
COPY --from=cargo-build /usr/src/myapp/target/my_target/release/udpdns .
CMD ["udpdns"]

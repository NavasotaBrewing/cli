# THANK YOU SO MUCH CAPNFABS
# https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/
FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libudev-dev:armhf

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
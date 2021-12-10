ARG OF_WATCHDOG_VERSION=0.9.2
ARG RUST_VERSION=1.56
ARG DEBIAN_VERSION=bullseye
ARG GST_PLUGINS_RS_VERSION=master

FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/openfaas/of-watchdog:${OF_WATCHDOG_VERSION} as watchdog
FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/kynothon/gst-plugins-rs-docker:${GST_PLUGINS_RS_VERSION} as gst-plugins-rs
FROM --platform=${TARGETPLATFORM:-linux/amd64} rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder

COPY --from=gst-plugins-rs / /

ARG ADDITIONAL_PACKAGE
ARG ADDITIONAL_DEV_PACKAGE

RUN apt-get -qy update && apt-get -qy install \
	libgstreamer1.0-dev \
	${ADDITIONAL_DEV_PACKAGE}

ENV PATH=$PATH:/home/app/.local/bin

WORKDIR /home/rust/

COPY main     ./main
COPY function ./function

RUN  cd main && cargo build --release

FROM --platform=${TARGETPLATFORM:-linux/amd64} debian:${DEBIAN_VERSION}-slim as runner

ARG ADDITIONAL_PACKAGE

# Install packages and add non-root user
RUN apt-get -qy update && apt-get -qy install \
	libgstreamer1.0-0 \
	streamer1.0-plugins-base \
	gstreamer1.0-plugins-good \
	gstreamer1.0-plugins-bad \
	gstreamer1.0-plugins-ugly \
	gstreamer1.0-libav \
	libcsound64-6.0 \
	libwebpdemux2 \
	${ADDITIONAL_PACKAGE} \
	&& rm -rf /var/lib/apt/lists/* \
	&& addgroup --system app \
	&& adduser app --system --home /home/app --ingroup app

WORKDIR /home/app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
COPY --from=gst-plugins-rs / / 
COPY --from=builder /home/rust/main/target/release/main /home/app/main

USER app

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

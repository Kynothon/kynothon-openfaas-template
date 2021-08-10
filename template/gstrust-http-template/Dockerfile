# syntax = docker/dockerfile:experimental
ARG OF_WATCHDOG_VERSION=0.8.1
ARG IMAGE=alpine
ARG IMAGE_VERSION=3.14

FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/openfaas/of-watchdog:${OF_WATCHDOG_VERSION} as watchdog
FROM ${IMAGE}:${IMAGE_VERSION} as builder

ENV CARGO_HOME=/root/.cargo
ARG DEV_PACKAGE

WORKDIR /home/rust

RUN apk add --no-cache rust \
		       cargo \
               glib-dev \
               gstreamer-dev \
               gst-plugins-base-dev \
		       ${DEV_PACKAGE}	

# Copy all the sources
COPY main ./main
RUN USER=nobody cargo init --lib --name handler function && \
    mkdir -p main/.cargo && \
    cd main && \
    cargo fetch

COPY function ./function
RUN  cd main && cargo build --release

FROM ${IMAGE}:${IMAGE_VERSION} as runner

ARG ADDITIONAL_PACKAGE

# Install packages and add non-root user
RUN apk --no-cache add libgcc \
                        ca-certificates \
                        gst-plugins-base \
                        glib \
                        gstreamer \
                        gst-plugins-base-dev \
                        gst-plugins-good \
                        gst-plugins-bad \
                        gst-plugins-ugly \
                        ${ADDITIONAL_PACKAGE} \
    && addgroup -S app && adduser -S -g app app

WORKDIR /home/app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
COPY --from=builder /home/rust/main/target/release/main /home/app/main

USER app

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]

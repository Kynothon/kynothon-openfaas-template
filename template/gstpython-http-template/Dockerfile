ARG OF_WATCHDOG_VERSION=0.9.2
ARG PYTHON_VERSION=3.9 
ARG DEBIAN_VERSION=bullseye          

FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/openfaas/of-watchdog:${OF_WATCHDOG_VERSION} as watchdog
FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/kynothon/gst-plugins-rs-docker:latest as gst-plugins-rs
FROM --platform=${TARGETPLATFORM:-linux/amd64} python:${PYTHON_VERSION}-slim-${DEBIAN_VERSION}

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

COPY --from=gst-plugins-rs / /

ARG ADDITIONAL_PACKAGE
# Alternatively use ADD https:// (which will not be cached by Docker builder)

RUN apt-get -qy update && apt-get install -y --no-install-recommends \
  libgstreamer1.0-0 \
  gstreamer1.0-plugins-base \
  gstreamer1.0-plugins-good \
  gstreamer1.0-plugins-bad \
  gstreamer1.0-plugins-ugly \
  gstreamer1.0-libav \
  python3-gst-1.0 \
  libcsound64-6.0 \
  libwebpdemux2 \
  ${ADDITIONAL_PACKAGE} \
  && rm -rf /var/lib/apt/lists/*

# Add non root user
RUN addgroup --system app && adduser app --system --ingroup app
RUN chown app /home/app

USER app

ENV PATH=$PATH:/home/app/.local/bin

WORKDIR /home/app/

COPY index.py           .
COPY requirements.txt   .

USER root
RUN pip install --disable-pip-version-check -r requirements.txt

# Build the function directory and install any user-specified components
USER app

RUN mkdir -p function
RUN touch ./function/__init__.py
WORKDIR /home/app/function/
COPY function/requirements.txt	.
RUN pip install --disable-pip-version-check --user -r requirements.txt

#install function code
USER root

COPY function/   .
RUN chown -R app:app ../

ENV PYTHONPATH=/usr/lib/python3/dist-packages

ARG TEST_COMMAND=tox
ARG TEST_ENABLED=true
RUN if [ "$TEST_ENABLED" == "false" ]; then \
    echo "skipping tests";\
    else \
    eval "$TEST_COMMAND"; \
    fi

WORKDIR /home/app/

#configure WSGI server and healthcheck
USER app

ENV fprocess="python index.py"

ENV cgi_headers="true"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:5000"

HEALTHCHECK --interval=5s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
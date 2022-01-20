ARG OF_WATCHDOG_VERSION=0.8.4
ARG PYTHON_VERSION=3.10 
ARG DEBIAN_VERSION=bullseye          
ARG BENTO4_VERSION=1.6.0-639

FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/openfaas/of-watchdog:${OF_WATCHDOG_VERSION} as watchdog
FROM --platform=${TARGETPLATFORM:-linux/amd64} ghcr.io/kynothon/bento4docker:${BENTO4_VERSION} as bento4
FROM python:${PYTHON_VERSION}-slim-${DEBIAN_VERSION}

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

COPY --from=bento4 /opt/bento4 /opt/bento4

ARG ADDITIONAL_PACKAGE
# Alternatively use ADD https:// (which will not be cached by Docker builder)

RUN apt-get -qy update && apt-get install -y --no-install-recommends \
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
RUN pip install -r requirements.txt

# Build the function directory and install any user-specified components
USER app

RUN mkdir -p function
RUN touch ./function/__init__.py
WORKDIR /home/app/function/
COPY function/requirements.txt	.
RUN pip install --user -r requirements.txt

#install function code
USER root

COPY function/   .
RUN chown -R app:app ../

ENV PYTHONPATH=/opt/bento4/utils:/usr/lib/python3/dist-packages

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

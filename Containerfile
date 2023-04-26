FROM ghcr.io/ctron/trunk:latest as builder

RUN mkdir /usr/src/console

COPY . /usr/src/console

WORKDIR /usr/src/console

RUN true \
    && npm ci \
    && trunk build \
    && mv dist public

FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

RUN true \
    && mkdir /public

COPY --from=builder /usr/src/console/public /

RUN chmod a+x /nginx.sh
CMD ["/nginx.sh"]
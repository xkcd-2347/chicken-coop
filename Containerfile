FROM ghcr.io/ctron/trunk:latest as builder

RUN mkdir /usr/src/console

COPY . /usr/src/console

WORKDIR /usr/src/console

RUN true \
    && npm ci \
    && trunk build --release --dist /public

FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

RUN microdnf install -y nginx

RUN true \
    && mkdir /public

COPY --from=builder /public /usr/share/nginx/html/

EXPOSE 80

CMD ["/usr/sbin/nginx", "-g", "daemon off;"]

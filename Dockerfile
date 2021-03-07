FROM rust:1.50.0-alpine3.13 AS build

WORKDIR /build
COPY . .

RUN apk -U upgrade && apk add mariadb-connector-c-dev musl-dev openssl-dev pkgconf \
    && rm -rf /var/cache/apk/*

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

FROM alpine:3.13 AS runtime

RUN apk -U upgrade && apk add mariadb-connector-c mariadb-client openssl ca-certificates \
    && rm -rf /var/cache/apk/*

WORKDIR /app
COPY --from=build /build/target/release/kit-newcomer-server /app/kit-newcomer-server

EXPOSE 8080
VOLUME [ "/assets" ]

CMD /app/kit-newcomer-server
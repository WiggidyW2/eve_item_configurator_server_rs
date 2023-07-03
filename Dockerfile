# FROM rust:1.69-bullseye
# COPY ./src src
# COPY ./Cargo.toml Cargo.toml
# RUN curl https://github.com/WiggidyW/eve_item_configurator_sqlite_accessor_rs/raw/master/db.sqlite -o db.sqlite -L
# RUN mv db.sqlite /root/db.sqlite
# RUN cargo build --release
# RUN mv target/release/wetc_item_configurator_server /root/service

# FROM frolvlad/alpine-glibc:alpine-3.17
# COPY --from=0 /root/service service
# COPY --from=0 /root/db.sqlite db.sqlite
# RUN apk add openssl1.1-compat
# RUN chmod +x service
# ENV SERVICE_ADDRESS=0.0.0.0:9090
# CMD ["./service"]

FROM rust:1.69-bullseye
COPY ./src src
COPY ./Cargo.toml Cargo.toml
RUN cargo build --release
RUN mv target/release/wetc_item_configurator_server /root/service

FROM frolvlad/alpine-glibc:alpine-3.17
COPY ./db.sqlite db.sqlite
COPY --from=0 /root/service service
RUN apk add openssl1.1-compat
RUN chmod +x service
ENV SERVICE_ADDRESS=0.0.0.0:9090
CMD ["./service"]

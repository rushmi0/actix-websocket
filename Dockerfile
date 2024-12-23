FROM rust:1.83.0 as build
WORKDIR /builder

COPY ./src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates adduser && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 rustapp && \
    useradd -u 1000 -g rustapp -m -s /bin/bash runner

USER runner
WORKDIR /app

COPY --from=build --chown=runner:rustapp /builder/target/release/actix-rs ./
COPY --chown=runner:nostrapp .env /app/.env

RUN chmod u=rx,g=r,o= /app/actix-rs && \
    chmod u=r,g=,o= /app/.env

EXPOSE 8080

ENTRYPOINT ["./actix-rs"]
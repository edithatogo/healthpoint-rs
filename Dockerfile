# syntax=docker/dockerfile:1
FROM rust:1-bookworm AS dev

WORKDIR /workspace
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        pkg-config \
        bash \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo fetch

CMD ["bash"]

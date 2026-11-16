# syntax=docker/dockerfile:1
FROM rust:1-bookworm@sha256:ade39a710ef85f3506cffcfec3e8a24ba090badef1dff2fb798f71d878833fea AS builder

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
RUN cargo build --locked --release -p healthpoint-mcp

FROM debian:bookworm-slim@sha256:734a72b3d6142a71993b4bc580ca01d8ca687f77a2c422d96a961d3376b179bc AS runtime

LABEL org.opencontainers.image.source="https://github.com/edithatogo/healthpoint-rs"
LABEL org.opencontainers.image.description="Read-only MCP server for licensed Healthpoint HL7 FHIR API access."
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL io.modelcontextprotocol.server.name="io.github.edithatogo/healthpoint-rs"

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && groupadd -r mcp \
    && useradd -r -g mcp mcp \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/healthpoint-mcp /usr/local/bin/healthpoint-mcp

USER mcp

ENTRYPOINT ["/usr/local/bin/healthpoint-mcp"]

FROM builder AS dev
CMD ["bash"]

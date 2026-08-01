FROM rust:nightly-bookworm AS builder
WORKDIR /app
RUN rustup toolchain install nightly-2026-02-18 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2026-02-18 && \
    rustup default nightly-2026-02-18
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY packages/ packages/
RUN cargo build --release --package montrs-cli && \
    cp target/release/montrs /usr/local/bin/montrs

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/montrs /usr/local/bin/montrs
COPY --from=builder /app/templates /templates
ENTRYPOINT ["montrs"]
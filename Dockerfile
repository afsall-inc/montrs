FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
COPY . .
RUN rustup toolchain install nightly-2026-02-18 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2026-02-18 && \
    cargo +nightly-2026-02-18 build --release --package montrs-cli && \
    cp target/release/montrs /usr/local/bin/montrs

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/montrs /usr/local/bin/montrs
COPY --from=builder /app/templates /templates
ENTRYPOINT ["montrs"]
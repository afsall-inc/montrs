FROM rust:1.84-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --package montrs-cli && \
    cp target/release/montrs /usr/local/bin/montrs

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/montrs /usr/local/bin/montrs
COPY --from=builder /app/templates /templates
ENTRYPOINT ["montrs"]
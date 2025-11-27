FROM rust:1.82 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
RUN apt update && apt install -y ca-certificates
COPY --from=builder /app/target/release/pappap-ai-chain /usr/local/bin/
COPY --from=builder /app/models /models
COPY --from=builder /app/web /web
CMD ["pappap-ai-chain"]
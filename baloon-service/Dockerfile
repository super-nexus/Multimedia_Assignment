# Stage 1: Building the binary
FROM rust:1.67 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

# Stage 2: Preparing the runtime environment
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/myapp/target/release/baloon-service /usr/local/bin/baloon-service

CMD ["baloon-service"]


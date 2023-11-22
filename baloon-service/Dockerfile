# Stage 1: Building the binary

# Use the official Rust image as a builder
FROM rust:1.56 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin my_rust_service
WORKDIR /my_rust_service

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build
RUN rm src/*.rs

# Now that dependencies are cached, copy your source code
COPY ./src ./src

# Build your application
RUN cargo build

# Stage 2: Preparing the runtime environment

# Use a smaller image to reduce size
FROM debian:buster-slim

# Copy the build artifact from the build stage
COPY --from=builder /my_rust_service/target/release/my_rust_service .

# Set the binary as the entrypoint of the container
ENTRYPOINT ["./my_rust_service"]

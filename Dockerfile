# # Stage 1: Build the Rust application
# FROM rust:1.83 as builder

# # Set the working directory
# WORKDIR /usr/src/app

# # Copy Rust project files
# COPY . .

# # Build the Rust application in release mode
# RUN cargo build --release
 
# # Stage 2: Create a minimal runtime image
# FROM debian:bookworm-slim

# # Install required runtime dependencies
# RUN apt-get update && \
#     apt-get install -y \
#     libssl-dev \
#     curl \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

# # Set working directory
# WORKDIR /app

# # Copy the compiled binary from the builder stage
# COPY --from=builder /usr/src/app/target/release/my-rhat-app .

# # Expose the application port
# EXPOSE 3040

# # Command to run the application
# CMD ["./my-rhat-app"]

# Stage 1: Build the Rust application
FROM rust:1.83 as builder
WORKDIR /usr/src/app

# Copy Rust project files
COPY . .
RUN cargo build --release && strip target/release/my-rhat-app

# Stage 2: Minimal Runtime with Distroless
FROM gcr.io/distroless/cc
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/my-rhat-app .

# Expose the application port
EXPOSE 3040

# Command to run the application
CMD ["/app/my-rhat-app"]

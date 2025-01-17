# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

# WORKDIR /app

# # stage 1, prepare the recipe for build caching
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# # stage 2, copy over source code and build
# FROM chef AS rust_builder
# COPY --from=planner /app/recipe.json recipe.json

# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json

# # Build application
# COPY . .
# RUN cargo build --release --bin my-rhat-app

# # stage 2b, build our css as we don't have a formal preprocessor
# FROM node:buster-slim as node_builder

# WORKDIR /app

# # we'll use pnpm to ensure we're consistent across the dev and release environments
# RUN corepack enable

# # copy on over all the dependencies
# COPY tailwind.config.cjs .
# COPY styles ./styles
# COPY assets ./assets

# # we'll also copy the templates over so tailwind can scan for unused class utilities, omitting them from the final output
# COPY ./templates ./templates

# # build our css
# RUN npx tailwindcss -i ./styles/input.css -o ./assets/output.css

# # stage 3, copy over our build artifacts and run
# # We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime

# WORKDIR /app

# # we'll copy over the executable from our server builder and the compiled tailwind assets separately - layer caching FTW!
# COPY --from=rust_builder /app/target/release/my-rhat-app ./server
# COPY --from=node_builder /app/assets ./assets

# EXPOSE 80
# EXPOSE 443

# ENTRYPOINT ["/app/server"]

# Stage 1: Build the Rust application
FROM rust:1.83 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy Rust project files
COPY . .

# Build the Rust application in release mode
RUN cargo build --release
 
# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/my-rhat-app .

# Expose the application port
EXPOSE 3040

# Command to run the application
CMD ["./my-rhat-app"]


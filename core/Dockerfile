# Use the official Rust image as the base
FROM rust:1.83 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock
COPY Cargo.toml ./

# Fetch dependencies and build the project
# RUN cargo fetch

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a smaller image for the final application
FROM debian:latest

# Install MariaDB client libraries
RUN apt-get update && apt-get install -y \
    libmariadb-dev-compat \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/basable .

# Set the default command to run the application
CMD ["./basable"]

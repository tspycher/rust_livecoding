# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy all the needed files to build the application
COPY Cargo.toml Cargo.lock diesel.toml db.sqlite ./
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Start with a minimal image to reduce size
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/tedtalk .
COPY --from=builder /app/diesel.toml .
COPY --from=builder /app/db.sqlite .

# Expose any necessary ports
EXPOSE 3000

# Command to run the application
CMD ["./tedtalk"]

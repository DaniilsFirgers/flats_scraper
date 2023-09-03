# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app


# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY scraper/Cargo.toml scraper/Cargo.lock ./

# Copy the entire project directory into the container
COPY scraper/ ./scraper/

# Specify the entry point for the container
CMD ["cargo", "run", "--release"]
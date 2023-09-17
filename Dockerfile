# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY scraper/Cargo.toml scraper/Cargo.lock /app/

# Copy the entire project directory into the container
COPY ./scraper/src /app/src
COPY ./scraper/data /app/data

# Specify the entry point for the container
CMD ["cargo", "run", "--release", "--bin", "ads-scraper-app"]
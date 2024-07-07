# Use the official Rust image as a parent image
FROM rust:1.70

# Set the working directory in the container
WORKDIR /usr/src/phantom_whisperer

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Make port 3030 available to the world outside this container
EXPOSE 3030

# Run the binary
CMD ["./target/release/phantom_whisperer"]

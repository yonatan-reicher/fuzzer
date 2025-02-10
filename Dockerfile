FROM rust:latest as rust-env

# Install Python alongside Rust
RUN apt-get update
RUN apt-get install -y python3
RUN apt-get install -y python3-pip
RUN apt-get install -y cmake
RUN apt-get install -y vim
RUN apt-get install -y dos2unix

# Add a non-root user for safety
RUN useradd -ms /bin/bash sandboxuser

# Install just the dependencies first
WORKDIR /home/sandboxuser
COPY ./Cargo.toml ./fuzzer/Cargo.toml
COPY ./Cargo.lock ./fuzzer/Cargo.lock
COPY ./src/main.rs ./fuzzer/src/main.rs
WORKDIR /home/sandboxuser/fuzzer
RUN cargo build --release; exit 0

# Copy the rest of the files
WORKDIR /home/sandboxuser
COPY . ./fuzzer
# Convert all files to Unix format
RUN find . -type f -print0 | xargs -0 dos2unix
RUN chown -R sandboxuser:sandboxuser /home/sandboxuser/fuzzer

WORKDIR /home/sandboxuser/fuzzer
USER sandboxuser

# Build
RUN cargo build --release
# Allow us to run these scripts
RUN chmod +x ./resources/ --recursive

# Default CMD to run the fuzzer
CMD ["cargo", "run", "--release"]

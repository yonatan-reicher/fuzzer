FROM rust:latest as rust-env

# Install Python alongside Rust
RUN apt-get update && apt-get install -y python3 python3-pip

# Add a non-root user for safety
RUN useradd -ms /bin/bash sandboxuser

WORKDIR /home/sandboxuser

COPY . ./fuzzer
RUN chown -R sandboxuser:sandboxuser /home/sandboxuser/fuzzer

WORKDIR /home/sandboxuser/fuzzer
USER sandboxuser

RUN cargo build --release

# Default CMD to run the fuzzer
CMD ["cargo", "run", "--release"]

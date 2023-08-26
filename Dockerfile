
# Build Stage
FROM rust:1.71 as chef

# Install cargo-chef to manage dependencies
RUN cargo install cargo-chef

# Set up the working directory
WORKDIR /rust_blog

# Intermediate stage for preparing dependency information
FROM chef AS planner

# Copy source code from previous stage
COPY . .

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Generate information for caching dependencies using cargo-chef
RUN cargo chef prepare --recipe-path recipe.json

# Intermediate stage for building and caching dependencies
FROM chef AS builder
COPY --from=planner /rust_blog/recipe.json recipe.json

# Build and cache dependencies using the prepared recipe
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the source code from the planner stage (update if necessary)
COPY . .

# Build application
RUN cargo build --release

# Deploy Stage
FROM gcr.io/distroless/cc-debian11

# Copy `libpq` dependencies into the image (Required by diesel)
ARG ARCH=x86_64

# libpq related (required by diesel)
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
COPY --from=builder /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/

# Application files
COPY --from=builder /rust_blog/target/release/ /usr/src/rust_blog

# Copy Rocket.toml
COPY ./Rocket.toml .

# Run the binary
CMD ["./usr/src/rust_blog/main"]
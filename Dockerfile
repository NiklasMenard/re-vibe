# 1. This tells docker to use the Rust official image
FROM rust:1.71 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin rust_blog
WORKDIR /rust_blog

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY . .

# 5. Build for release.
RUN rm ./target/release/deps/rust_blog*
RUN cargo build --release

# our final base
FROM rust:1.71

# copy the build artifact from the build stage
COPY --from=build /rust_blog/target/release/ /target/release/.

# Copy Rocket.toml
COPY ./Rocket.toml ./Rocket.toml

# Run the binary
CMD ["./target/release/main"]
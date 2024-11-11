# --- Build container  ---
FROM rust:bookworm as build

# Pre-compile dependencies
RUN USER=root cargo new --bin neilwoodhouseuk
WORKDIR /neilwoodhouseuk
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Copy in actual source code
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

# Build Application
RUN rm ./target/release/deps/neilwoodhouseuk*
RUN cargo build --release



# --- Web Container ---
FROM nginx:bookworm

WORKDIR /APP

COPY --from=build /neilwoodhouseuk/target/release/neilwoodhouseuk bin/
COPY ./neilwoodhouse.uk.conf /etc/nginx/conf.d/default.conf
COPY ./entrypoint.sh ./
CMD ["/bin/sh", "/APP/entrypoint.sh"]
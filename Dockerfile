# Stage 1

FROM rust:slim-bullseye as builder

#Setting work directory

WORKDIR /usr/src/app

COPY . . 

RUN cargo build --release

# COPY Cargo.toml Cargo.lock ./

# COPY src ./src

# COPY ./target/release/edupro_backend /usr/local/bin/edupro_backend


# RUN apt-get update && apt-get install -y libssl-dev pkg-config

# RUN cargo build --release


# CMD ["./target/release/edupro_backend"]


# Stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/edupro_backend .
EXPOSE 8000

CMD ["./edupro_backend"]
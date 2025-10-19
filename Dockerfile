FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

ARG CRATE_NAME

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin $CRATE_NAME

FROM gcr.io/distroless/static:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/$CRATE_NAME /usr/bin
ENTRYPOINT ["/usr/bin/$CRATE_NAME"]
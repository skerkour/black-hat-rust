####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

WORKDIR /ch_11

COPY ./ .

RUN cargo build -p server --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Create unprivileged user
ENV USER=ch_11
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /ch_11

# Copy our build
COPY --from=builder /ch_11/target/release/server ./

# Use an unprivileged user
USER ch_11:ch_11

CMD ["/ch_11/server"]

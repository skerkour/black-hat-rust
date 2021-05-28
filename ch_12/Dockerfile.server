####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

WORKDIR /ch12

COPY ./ .

RUN cargo build -p server --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Create unprivileged user
ENV USER=ch12
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /ch12

# Copy our build
COPY --from=builder /ch12/target/release/server ./

# Use an unprivileged user
USER ch12:ch12

CMD ["/ch12/server"]

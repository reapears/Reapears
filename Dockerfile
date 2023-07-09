# Build server
FROM rust:latest AS builder_rust

RUN rustup update
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools
RUN update-ca-certificates

# Create appuser
ENV USER=reapears
ENV UID=10001

# See https://stackoverflow.com/a/55757473/12429735RUN
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /reapears

COPY ./ .
WORKDIR /reapears/reapears

ENV SQLX_OFFLINE true

RUN cargo build --target x86_64-unknown-linux-musl --release
# RUN make build_static # equivalent to cargo build --target x86_64-unknown-linux-musl --release

# Final image
FROM debian:buster-slim

# Import from builder.
COPY --from=builder_rust /etc/passwd /etc/passwd
COPY --from=builder_rust /etc/group /etc/group

WORKDIR /reapears


# Copy our builds
# COPY --from=builder_rust /reapears/reapears/dist/ ./
COPY --from=builder_rust /reapears/reapears/target/release/reapears ./

# Use an unprivileged user.
USER reapears:reapears

EXPOSE 8080 8443
CMD ["/reapears/reapears", "server"] 
# If some crashes or slowness are noticed when running the static rust binary with musl and Jemalloc
# see here: https://andygrove.io/2020/05/why-musl-extremely-slow/
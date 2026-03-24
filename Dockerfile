FROM rust:latest AS builder
LABEL "description"="Photos.network frontend"
LABEL "version"="0.3.0"
LABEL "maintainer"="github.com/photos-network"

RUN cargo install cargo-leptos --locked

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod +x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

ENV USER=frontend
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /frontend

COPY ./ .

RUN cargo leptos build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /frontend

COPY --from=builder /frontend/target/release/frontend ./frontend
COPY --from=builder /frontend/target/site ./target/site

USER frontend:frontend

ENV LEPTOS_SITE_ADDR="0.0.0.0:7778"
ENV LEPTOS_SITE_ROOT="target/site"

EXPOSE 7778

CMD ["/frontend/frontend"]

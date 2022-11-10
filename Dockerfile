FROM rust:1.65-bullseye as builder

ARG PORT=3000
ARG REDIS_URL
ARG PASSWD_SECRET
ENV PORT=${PORT}
ENV REDIS_URL=${REDIS_URL}
ENV PASSWD_SECRET=${PASSWD_SECRET}

COPY . .
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/main_server /usr/local/bin/main_server
ENTRYPOINT ["main_server"]

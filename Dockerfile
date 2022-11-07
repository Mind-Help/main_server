FROM rust:1.65-bullseye as builder

ARG PORT=3000
ARG REDIS_URL
ARG PASSWD_SECRET
ENV PORT=${PORT}
ENV REDIS_URL=${REDIS_URL}
ENV PASSWD_SECRET=${PASSWD_SECRET}

WORKDIR /opt/main_server
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /opt/main_server
COPY --from=builder /opt/main_server/target/release/main_server .
ENTRYPOINT ["main_server"]
FROM rust:latest as builder

WORKDIR /usr/app/
COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/app/

COPY --from=builder /usr/app/target/release/ .
RUN chmod +x ./discord_bot

CMD ["./discord_bot"]

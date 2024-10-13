FROM rust as builder

WORKDIR /usr/src/calypso
COPY . .

RUN apt-get update && apt-get install -y libssl-dev build-essential cmake 
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt update
RUN apt install openssl -y

COPY --from=builder /usr/local/cargo/bin/calypso /usr/local/bin/calypso
COPY --from=builder /usr/local/cargo/bin/simulate /usr/local/bin/simulate

CMD ["simulate"]

# START WITH: sudo docker run -d --rm  --network host calypso
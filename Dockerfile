FROM rust AS builder

WORKDIR /usr/src/calypso
COPY . .

RUN git submodule update --init
RUN apt-get update && apt-get install -y libssl-dev build-essential cmake 
RUN cargo install --path  --locked --bins .

FROM debian:bookworm-slim
RUN apt update
RUN apt install openssl -y

COPY --from=builder /usr/local/cargo/bin/calypso /usr/local/bin/calypso
COPY --from=builder /usr/local/cargo/bin/calypso-simulate /usr/local/bin/calypso-simulate

CMD ["calypso-simulate"]

# START WITH: sudo docker run -d --rm  --network host calypso
FROM rust AS builder

WORKDIR /usr/src/calypso
COPY . .

RUN git submodule update --init
RUN apt-get update && apt-get install -y libssl-dev build-essential cmake 
RUN cargo build --all --release

FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install openssl -y

COPY --from=builder /usr/src/calypso/target/release/calypso /usr/local/bin/calypso
COPY --from=builder /usr/src/calypso/target/release/calypso-simulate /usr/local/bin/calypso-simulate

CMD ["calypso-simulate"]

# START WITH: sudo docker run -d --rm  --network host calypso
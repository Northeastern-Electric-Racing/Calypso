FROM rust:bookworm AS builder

WORKDIR /usr/src/calypso
COPY . .

RUN git submodule update --init
RUN apt-get update && apt-get install -y libssl-dev build-essential cmake
RUN cargo install --path .
# The simulator was extracted into its own `calypso-sim` workspace member, so
# `cargo install --path .` no longer builds it — install it separately, as
# `simulate`, to preserve the historical entry point that downstream compose
# files invoke (Argos `compose.calypso.yml` relies on the default CMD below;
# Argos `compose.calypso.debug.sim.yml` and Nero use `command: ["simulate"]`).
RUN cargo install --path ./calypso-sim

FROM debian:bookworm-slim
RUN apt update
RUN apt install openssl -y

COPY --from=builder /usr/local/cargo/bin/calypso /usr/local/bin/calypso
COPY --from=builder /usr/local/cargo/bin/calypso-sim /usr/local/bin/simulate

CMD ["simulate"]

# START WITH: sudo docker run -d --rm  --network host calypso

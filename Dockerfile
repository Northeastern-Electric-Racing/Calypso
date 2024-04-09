FROM rust:1.70

WORKDIR /usr/src/calypso



# install dependenices
RUN apt-get update && apt-get install -y python3-pip cmake
RUN pip install ruamel.yaml==0.18.5


# build the project
COPY . .
RUN cargo build --release

CMD [ "./target/release/calypso", "mqtt", "localhost:1883", "vcan0", "skip_can_configure" ]
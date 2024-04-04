FROM rust:1.70

WORKDIR /usr/src/calypso

COPY . .

# install dependenices
RUN apt-get update && apt-get install -y python3-pip cmake
RUN pip install ruamel.yaml==0.18.5
RUN cargo install --path .


# build the project
RUN cargo build --release

CMD [ "calypso", "mqtt", "localhost:1883", "vcan0", "skip_can_configure" ]
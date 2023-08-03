FROM rust:1.70.0

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666

WORKDIR /app
COPY . .

RUN cargo install --path .

RUN rustup default nightly
RUN cargo build

CMD bash -c "diesel migration run && cargo run"
FROM rust:1.19.0

WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["app"]

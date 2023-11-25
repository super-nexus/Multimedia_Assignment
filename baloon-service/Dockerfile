FROM rust:1.67-slim-buster

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]
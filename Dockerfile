FROM rust:1.63.0

ENV MDB_URL default
ENV MDB_DATABASE_NAME default
ENV TELEGRAM_TOKEN default

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release

CMD ["cargo", "run"]
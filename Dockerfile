FROM rust

WORKDIR /usr/src/do-alerts-discord
COPY . .

RUN cargo install --path .

CMD ["do-alerts-discord"]

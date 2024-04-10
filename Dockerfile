FROM rust:1.75

WORKDIR /usr/src/telegram_new_features
COPY . .

RUN cargo install --path .

CMD [ "telegram_new_features" ]

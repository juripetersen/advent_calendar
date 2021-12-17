FROM rust:1.56.0

WORKDIR /var/www/html
COPY . .

RUN cargo install --path .

CMD ["advent_calendar-http"]

FROM rust:latest

WORKDIR /usr/src/todo-backend
COPY ./wait-for-it.sh /usr/wait-for-it.sh
COPY . .
RUN cargo install --path .
RUN chmod +x /usr/wait-for-it.sh

CMD ["/usr/wait-for-it.sh", "database:5432", "--", "todo-backend"]

docker run --name sqlx-test-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=myapp_test \
  -p 5432:5432 \
  -d postgres:18
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/myapp_test
sqlx database create
sqlx migrate run
cargo sqlx prepare
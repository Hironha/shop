insert into extra (id, name, price, created_at, updated_at)
values ($1, $2, $3, $4, $5)
returning id
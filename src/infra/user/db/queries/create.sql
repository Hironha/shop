insert into auth.user(id, username, email, email_verified, password_hash, created_at, updated_at)
values ($1, $2, $3, $4, $5, $6, $7)
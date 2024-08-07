select u.password_hash from auth.user as u
where u.email = $1
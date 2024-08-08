update auth.session
set exp = $1
where id = $2
update product
set name = $1, price = $2, kind = $3, updated_at = $4
where id = $5 and catalog_id = $6

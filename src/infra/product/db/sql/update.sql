update product
set name = $1, price = $2, updated_at = $3
where id = $4 and catalog_id = $5

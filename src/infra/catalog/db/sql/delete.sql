delete from catalog_with_products as catalog
where id = $1
returning catalog.*
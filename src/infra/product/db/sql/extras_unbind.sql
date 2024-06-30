delete from product_extras as pe
where pe.product_id = $1 and pe.extra_id not in (select * from unnest($2))
insert into product_extras (product_id, extra_id)
select * from unnest($1::uuid[], $2::uuid[])
on conflict (product_id, extra_id)
do nothing
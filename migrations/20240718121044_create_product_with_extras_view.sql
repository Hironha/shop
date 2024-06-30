-- Add migration script here

create or replace view product_with_extras as
select 
    product.*,
    coalesce(
        (select jsonb_agg(extra.*)
        from extra
        inner join product_extras as pe on pe.product_id = product.id),
        '[]'::jsonb
    ) as extras
from product;
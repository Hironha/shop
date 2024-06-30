-- Add migration script here

create or replace view catalog_with_products as
select
    catalog.*,
    coalesce(
        (select jsonb_agg(product.*)
        from product_with_extras as product
        where product.catalog_id = catalog.id),
        '[]'::jsonb
    ) as products
from catalog;
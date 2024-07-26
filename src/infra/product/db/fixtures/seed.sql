insert into catalog (id, name, description, created_at, updated_at)
values 
    ('0190ec30-286b-7211-aadb-003fc0449734', 'Burgers', null, now(), now()),
    ('0190ec30-7e38-75c0-a207-13c52449957d', 'Vegan', null, now(), now());

insert into product(id, catalog_id, name, price, kind, created_at, updated_at) 
values
    -- add to Burgers catalog
    ('0190ec14-0af8-71d1-9554-f1e5249ae3a2', '0190ec30-286b-7211-aadb-003fc0449734', 'Cheese Burger', 2000, 'burger', now(), now()),
    -- add to Vegan catalog
    ('0190ec15-7985-7e62-aaca-d65c07e6d2e5', '0190ec30-7e38-75c0-a207-13c52449957d', 'Caesar Salad', 1630, 'vegan', now(), now());

insert into extra(id, name, price, created_at, updated_at)
values 
    ('0190ec10-4aa7-7552-ba8f-df997d9f8a8e', 'Hot Sauce', 150, now(), now()),
    ('0190ec13-15cc-7f53-bc0f-d60f0beea824', 'Cheddar', 200, now(), now());

insert into product_extras (product_id, extra_id)
values 
    -- add Hot Sauce extra to Cheese Burger product 
    ('0190ec14-0af8-71d1-9554-f1e5249ae3a2', '0190ec10-4aa7-7552-ba8f-df997d9f8a8e');


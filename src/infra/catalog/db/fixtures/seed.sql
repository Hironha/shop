insert into catalog(id, name, description, created_at, updated_at)
values 
    ('0190ec30-286b-7211-aadb-003fc0449734', 'Burgers', 'Delicious burgers', now(), now()),
    ('0190ec30-7e38-75c0-a207-13c52449957d', 'Vegan', null, now(), now());

insert into product(id, catalog_id, name, price, kind, created_at, updated_at)
values 
    -- add Cheese Buregr to Burgers catalog
    ('0190ec14-0af8-71d1-9554-f1e5249ae3a2', '0190ec30-286b-7211-aadb-003fc0449734', 'Cheese Burger', 2000, 'burger', now(), now());
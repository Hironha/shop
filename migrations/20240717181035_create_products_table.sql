-- Add migration script here

create table if not exists extra (
    id uuid,
    name varchar(128) not null,
    price decimal(20, 2) not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    constraint pk_extra primary key (id),
    constraint ak_extra_name unique (name)
);

create table if not exists catalog (
    id uuid,
    name varchar(64) not null,
    description varchar(128),
    created_at timestamptz not null,
    updated_at timestamptz not null,

    constraint pk_catalog primary key (id),
    constraint ak_catalog_name unique (name)
);

create table if not exists product (
    id uuid,
    catalog_id uuid not null,
    name varchar(64) not null,
    price decimal(20, 2) not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    constraint pk_product primary key (id),
    constraint ak_product_name unique (name, catalog_id),
    constraint fk_product_catalog_id 
        foreign key (catalog_id) references catalog (id) on delete cascade
);

create table if not exists product_extras (
    product_id uuid,
    extra_id uuid,

    constraint pk_product_extras_id primary key (product_id, extra_id),
    constraint fk_product_extras_product_id 
        foreign key (product_id) references product (id) on delete cascade,
    constraint fk_product_extras_extra_id 
        foreign key (extra_id) references extra (id) on delete cascade
)
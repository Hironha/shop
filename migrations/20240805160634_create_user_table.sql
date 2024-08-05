-- Add migration script here

create schema if not exists auth authorization root;

create table if not exists auth.user (
    id UUID,
    username varchar(64) not null,
    email varchar(256) not null,
    password_hash text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,
    constraint pk_user primary key (id),
    constraint ak_user_email unique (email) 
);
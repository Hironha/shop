-- Add migration script here

create schema if not exists auth authorization root;

create table if not exists auth.user (
    id uuid,
    username varchar(64) not null,
    email varchar(256) not null,
    email_verified boolean not null,
    password_hash text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,
    constraint pk_user primary key (id),
    constraint ak_user_email unique (email) 
);

create table if not exists auth.session (
    id uuid,
    user_id uuid,
    iss timestamptz not null,
    exp timestamptz not null,
    constraint pk_session primary key (id, user_id),
    constraint fk_session_user_id foreign key (user_id) references auth.user(id)
);
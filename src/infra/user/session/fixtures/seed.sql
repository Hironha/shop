insert into auth.user(id, username, email, email_verified, password_hash, created_at, updated_at)
values 
    (
        '019128e9-f215-7f81-b053-e8edd437df24', 
        'test', 
        'test@test.com', 
        true, 
        'test', 
        now(), 
        now()
    ),
    (
        '01912f17-e118-7ec1-9aff-bd7d5636d1fc', 
        'marcus', 
        'marcus@gmail.com', 
        true, 
        'marcus', 
        now(), 
        now()
    );


insert into auth.session(id, iss, exp)
values 
    ('019128e9-f215-7f81-b053-e8edd437df24', now(), now() + interval '2h');
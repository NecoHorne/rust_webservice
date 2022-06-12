create table password_reset_token
(
    id         bigint       not null
        primary key,
    expiryDate datetime(6)  null,
    token      varchar(255) null,
    user_id    int unsigned not null,
    constraint FK83nsrttkwkb6ym0anu051mtxn
        foreign key (user_id) references users (id)
);
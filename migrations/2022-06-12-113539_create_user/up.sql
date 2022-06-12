-- Your SQL goes here
create table users
(
    id                 int unsigned auto_increment
        primary key,
    date_created       timestamp                   null,
    date_updated       timestamp                   null,
    uid                char(36)                    not null,
    display_name       varchar(255)                null,
    first_name         varchar(255)                null,
    last_name          varchar(255)                null,
    email              varchar(255)                not null,
    email_verified     tinyint(1)     default 0    not null,
    telephone          varchar(255)                null,
    last_login         timestamp                   null,
    fcm                varchar(255)                null,
    is_banned          tinyint(1)     default 0    not null,
    ip_address         varchar(255)                null,
    meta_data          varchar(255)                null,
    birthday           timestamp                   null,
    password           varchar(255)                not null,
    role               int            default 1    not null,
    telephone_verified tinyint(1)     default 0    not null,
    user_rating        decimal(10, 2) default 0.00 not null,
    constraint users_email_unique
        unique (email),
    constraint users_uid_unique
        unique (uid)
);
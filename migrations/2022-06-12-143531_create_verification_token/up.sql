-- Your SQL goes here
create table verification_token
(
    id         bigint       not null
        primary key,
    expiryDate datetime(6)  null,
    token      varchar(255) null,
    user_id    int unsigned not null,
    constraint FK3asw9wnv76uxu3kr1ekq4i1ld
        foreign key (user_id) references users (id)
);
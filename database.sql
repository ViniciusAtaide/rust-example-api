
drop table if exists todo_list;
drop table if exists todo_item;
drop table if exists post;

create table post (
       id serial primary key,
       title varchar(150) not null,
       subtitle varchar(150),
       image_url varchar(150)
);



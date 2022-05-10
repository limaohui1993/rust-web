drop table if exists test_lmh;

create table test_lmh (
    id serial primary key,
    teacher_id INT not null,
    name VARCHAR(140) not null,
    time TIMESTAMP default now()
)

insert into test_lmh
    (id, teacher_id,name, time)
values (1,1,'First course','2022-01-17 05:40:00');
insert into test_lmh
    (id, teacher_id,name, time)
values (2,1,'Second course','2022-01-18 05:45:00');    
drop table if exists file_info;
create table file_info (
    id bigint primary key,
    file_name varchar ,
    file_size bigint ,
    file_path varchar ,
    update_at timestamp ,
);

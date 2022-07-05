-- 自动更新时间触发器
create or replace function upd_timestamp() returns trigger as
$$
begin
    new.updated_at = current_timestamp;
return new;
end
$$
language plpgsql;

-- 联合索引：减少开销，一条抵 N 条。（参考 索引最左原则）
-- 文件表
drop table if exists files;
create table files
(
    id         bigint                    not null primary key,
    name       varchar(100)              not null,
    type       varchar(100)              not null,
    size       bigint      default 0     not null,
    path       varchar(500)              not null,
    times      bigint      default 0     not null,
    parent     varchar(500)              not null,
    created_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
);
create trigger user_timestamp before update on files for each row execute procedure upd_timestamp();
create index idx_name on files (name);
create index idx_type on files (type);
create index idx_size on files (size);
create index idx_times on files (times);
create index idx_parent on files (parent);
create index idx_created_at on files (created_at);
create index idx_updated_at on files (updated_at);
comment on table  files             is '文件表';
comment on column files.id          is '雪花主键';
comment on column files.name        is '文件名称';
comment on column files.type        is '文件类型：file、folder、symlink';
comment on column files.size        is '文件体积：单位 byte';
comment on column files.path        is '完整路径';
comment on column files.times       is '访问次数';
comment on column files.parent      is '上级主键';
comment on column files.created_at  is '创建时间';
comment on column files.updated_at  is '更新时间';

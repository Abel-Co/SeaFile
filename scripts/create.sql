-- 自动更新时间触发器
create or replace function upd_timestamp() returns trigger as
$$
begin
    new.updated_at = current_timestamp;
return new;
end
$$
language plpgsql;

-- 添加/删除 trigger
-- drop trigger user_timestamp on files;

-- 联合索引：减少开销，一条抵 N 条。（参考 索引最左原则）
-- 文件表
drop table if exists files;
create table files
(
    id         bigint                    not null primary key,
    kind       varchar(100)              not null,
    name       varchar(500)              not null,
    size       bigint      default 0     not null,
    path       varchar(1024)             not null,
    crc        bigint      default 0     not null,
    times      bigint      default 0     not null,
    parent     bigint      default 0     not null,
    created_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
);
create trigger file_timestamp before update on files for each row execute procedure upd_timestamp();
create index idx_name on files (name);
create index idx_kind on files (kind);
create index idx_size on files (size);
create index idx_path on files (path);
create index idx_crc on files (crc);
create index idx_times on files (times);
create index idx_parent on files (parent);
create index idx_created_at on files (created_at);
create index idx_updated_at on files (updated_at);
comment on table  files             is '文件表';
comment on column files.id          is '雪花主键';
comment on column files.name        is '文件名称';
comment on column files.kind        is '文件种类：file、folder、symlink';
comment on column files.size        is '文件体积：单位 byte';
comment on column files.path        is '完整路径';
comment on column files.times       is '访问次数';
comment on column files.parent      is '上级主键';
comment on column files.created_at  is '创建时间';
comment on column files.updated_at  is '更新时间';

-- 用户表
drop table if exists users;
create table users
(
    id         bigint                     not null primary key,
    username   varchar(500) unique        not null,
    password   varchar(500)               not null,
    email      varchar(320)                       ,
    avatar     varchar(500)                       ,
    phone      varchar(50)                        ,
    user_type  varchar(50)                        ,
    status     integer                            ,
    usage      bigint       default 0     not null,
    quota      bigint       default 1     not null ,
    created_at timestamptz  default now() not null,
    updated_at timestamptz  default now() not null
);

create trigger users_timestamp before update on users for each row execute procedure upd_timestamp();
create index idx_username on users (username);
comment on table  users             is '用户表';
comment on column users.id          is '主键: 雪花主键';
comment on column users.username    is '账号';
comment on column users.password    is '密码';
comment on column users.email       is '邮箱';
comment on column users.phone       is '电话';
comment on column users.avatar      is '头像：email、http:xxx';
comment on column users.user_type   is '用户类型：admin, user';
comment on column users.status      is '状态: 1.正常; 419.冻结';
comment on column users.quota       is '配额：单位GB，默认初始1GB';
comment on column users.usage       is '存储使用量';
comment on column users.created_at  is '创建时间';
comment on column users.updated_at  is '更新时间';

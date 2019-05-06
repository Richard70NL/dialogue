-- *****************************************************************************

create schema dialogue;

-- *****************************************************************************

create table t_group(
  f_group_id text not null,
  f_description text not null,
  f_sequence bigint not null default 0,
  constraint pk_group primary key(f_group_id)
);

-- *****************************************************************************

create table t_article(
  f_message_id text not null,
  f_body text not null,
  f_path text not null,
  f_from text not null,
  f_subject text not null,
  f_date timestamp not null,
  constraint pk_article primary key(f_message_id)
);

-- *****************************************************************************

create table t_header(
  f_message_id text not null,
  f_header text not null,
  f_value text not null,
  constraint pk_header primary key(f_message_id, f_header),
  constraint fk_header_1 foreign key(f_message_id) references t_article(f_message_id)
);

-- *****************************************************************************

create table t_group_article(
  f_group_id text not null,
  f_message_id text not null,
  f_number bigint not null,
  constraint pk_group_article primary key(f_group_id, f_message_id),
  constraint uk_group_article_1 unique(f_group_id, f_number),
  constraint fk_group_article_1 foreign key(f_group_id) references t_group(f_group_id),
  constraint fk_group_article_2 foreign key(f_message_id) references t_article(f_message_id)
);

-- *****************************************************************************

create or replace function dialogue.schema_version() returns integer as $$
  select 1;
$$ language sql;

-- *****************************************************************************

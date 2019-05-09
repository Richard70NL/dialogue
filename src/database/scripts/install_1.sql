-- *****************************************************************************

create schema dialogue;

-- *****************************************************************************

create table dialogue.t_group(
  f_group_id text not null,
  f_description text not null,
  f_sequence integer not null default 0,
  f_allowed bool not null default false,
  f_moderated bool not null default false,
  constraint pk_group primary key(f_group_id)
);

-- *****************************************************************************

create table dialogue.t_article(
  f_message_id text not null,
  f_body text not null,
  f_path text not null,
  f_from text not null,
  f_subject text not null,
  f_date timestamp not null,
  constraint pk_article primary key(f_message_id)
);

-- *****************************************************************************

create table dialogue.t_header(
  f_message_id text not null,
  f_header text not null,
  f_value text not null,
  constraint pk_header primary key(f_message_id, f_header),
  constraint fk_header_1 foreign key(f_message_id) references dialogue.t_article(f_message_id)
);

-- *****************************************************************************

create table dialogue.t_group_article(
  f_group_id text not null,
  f_message_id text not null,
  f_number integer not null,
  constraint pk_group_article primary key(f_group_id, f_message_id),
  constraint uk_group_article_1 unique(f_group_id, f_number),
  constraint fk_group_article_1 foreign key(f_group_id) references dialogue.t_group(f_group_id),
  constraint fk_group_article_2 foreign key(f_message_id) references dialogue.t_article(f_message_id)
);

-- *****************************************************************************

create or replace function dialogue.schema_version() returns integer as $$
  select 1;
$$ language sql;

-- *****************************************************************************

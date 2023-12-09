-- Add up migration script here
create table if not exists "user" (
  id uuid primary key  default uuid_generate_v1mc(),
  name varchar(255) collate "case_insensitive" unique not null,
  email varchar(255) collate "case_insensitive" unique not null,
  bio text default '',
  avatar varchar(255),
  passwd_hash varchar(255) not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);

-- applying our `updated_at` trigger
select trigger_updated_at('"user"');

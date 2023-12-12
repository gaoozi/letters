-- Add up migration script here
create table if not exists article (
  id uuid primary key   default uuid_generate_v1mc(),
  author_id uuid not null references "user" (id) on delete cascade,
  title text not null,
  slug text unique not null,
  description text not null,
  body text not null,
  created_at  timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

select trigger_updated_at('article');

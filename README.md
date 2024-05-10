# bleebo

static site upload and host service

**just for fun! not ready for production usage**

## sql tables

data is stored in a sqlite file named `db`. set up the tables using the
following commands:

```sql
create table users (
  username text primary key,
  password_hash text not null,
  reset_password boolean not null default false
);
pragma foreign_keys = on;
create table sites (
  site_name text primary key,
  owner_name text not null,
  foreign key(owner_name) references users(username)
);
```

# bleebo

static site upload and host service

**just for fun! not ready for production usage**

## sql tables

data is stored in a sqlite file

```sql
create table users (
  username text primary key,
  password_hash text not null,
  reset_password boolean not null default false
);
```

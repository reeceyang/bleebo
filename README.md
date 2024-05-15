# bleebo

upload static sites to [bleebo.dev](bleebo.dev) from the cli.

**just for fun! not for serious usage (yet??)**

## install
run `brew tap reeceyang/bleebo` and then `brew install bleebo`

## client usage
run `bleebo help` to see help

you should first change your password with `bleebo change-password`.

to upload a site, navigate to folder that you want to upload, then run `bleebo upload website-name`. 

> `website-name` must be all lowercase and should not contain any non-url safe characters like whitespace, otherwise it won't work (i know 💀 i will fix it). note that there is currently a 1 MiB file size limit, and dot files and folders are ignored

for example, to upload the contents of the `~/mywebsite` folder to the `hello.bleebo.dev` domain, you would run the following commands:

```bash
cd ~/mywebsite
bleebo upload hello
```

> **warning! everything uploaded will be public on the internet!**

## server usage
run the server with `bleebo server start`. insert a new user with `bleebo server new-user`. the server will try to serve files from the `./site` folder and store user info in the `./db` file. if these don't exist it probably won't work correctly. also note that the `db` file has to be set up according to the section below. 

ssl certificates should be configured since the client transmits auth credentials using http basic auth (i'm using certbot/let's encrypt).

### sql tables

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

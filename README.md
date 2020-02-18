## blog-api

This is the API behind [my personal webpage](https://victor.cloudflavor.io).

```bash
$ podman run -ti -p 5432:5432 -e POSTGRES_DB=blog -e POSTGRES_PASSWORD=123123 -e POSTGRES_USER=user postgres:11.5
...
```

The migrations are embeded at compile time.
Adding a new migration is done with the `diesel cli`.

```bash
$ diesel migration generate alter_posts
Creating migrations/2020-02-17-231108_alter_posts/up.sql
Creating migrations/2020-02-17-231108_alter_posts/down.sql
```

Updating the [schema.rs](src/schema.rs) file is done by running the migration
with the `diesel cli`. This will update the file based on the current schema
residing in the database.

```bash
 diesel migration run --database-url=postgres://root:pass@127.0.0.1/blog
```

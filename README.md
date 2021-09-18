# Diesel and PostgreSQL

## Diesel CLI

First we need to install the diesel cli to help us with some boilerplate's. To install CLI we need to install first the database driver, for linux just install:

```
sudo apt install libpq-dev
```

After that you can install diesel cli, using the command bellow. OBS: we going to install only the postgres features.

```
cargo install diesel_cli --no-default-features --features postgres
```

In my case, I use ASDF to install rust, so I need to run the command bellow to link the rust tool in my environment:

```
asdf reshim rustlang stable
```

Now i can use the CLI, run these two commands:

```
diesel setup
```

```
diesel migration generate MIGRATION_NANE
```

To run the migration:

```
diesel migration run
```

To undo the migration:

```
disel migration redo
```
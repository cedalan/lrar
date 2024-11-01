# hellooooooo

### prerequisites for running this shit:
---

ya need to create an .env file with the location of your db. (sqlite)
make the location relative to the cargo.toml file:

```
DATABASE_URL=./db/database.db
```

then ya prolly should create the database, I'm too stupid to do some kind of setup script. 

we're using [Diesel](https://diesel.rs/guides/getting-started)
so go take a look at that

should automagically run create table and migrations in the database for CI/CD, haven't gotten that far.
The tests require an empty database
also check [this out for how to compile rust stuff on ass berry pi](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050)
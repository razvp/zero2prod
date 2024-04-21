## To run locally
Launch PostgreSQL and Redis containers:
```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```
Lauch pgAdmin Docker container if you want to inspect the database:
```bash
./scripts/init_pgAdmin_container.sh

# open localhost:80 and login with user@domain.com (password: SuperSecret)
# add database
# use host.docker.internal as HostName to get the host address for the PostgreSQL container
```
Then:
```bash
# start server
cargo watch -x run 
curl localhost:8000/health_check -v # should return 200 OK
```
Now you can go to http://localhost:8000/login and login with `admin` `everythinghastostartsomewhere`.


## To test
Launch PostgreSQL and Redis containers:
```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```
Then `cargo test`.

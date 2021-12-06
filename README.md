# casql

### Local Testing
```bash
docker run --rm --name pg-test-db -p 5432:5432 \
-e POSTGRES_USER=root -e POSTGRES_PASSWORD=cascat -e POSTGRES_DB=dbname \
-v $(pwd)/tests/resources:/docker-entrypoint-initdb.d \
postgres:13
```

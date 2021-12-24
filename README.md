# casql

Query Postgres and get the results as JSON.

## Usage

### Querying
```bash
## Specify all params
casql query --host localhost --dbname dbname --username root --password $DB_PASSWORD "SELECT * FROM pg_type"

## Use saved params
casql query --conn test "SELECT * FROM pg_type"

## Use saved params but override some
casql query --conn test --port 5431 "SELECT * FROM pg_type"  
```

### Saved Connections
Connection parameters can be saved to make querying them simpler. Any subset of parameters can be saved, as long as all of them are provided at query time. 

The saved connections can be manipulated through the CLI or directly edited, as it is just a TOML file.
```bash
## Save new connection
casql conns save --host localhost --dbname dbname --username root --name test-conn 

## List saved connections
casql conns list

## Show saved connection
casql conns describe --name test

## Delete saved connection
casql conns delete --name test
```

## Postgis
`casql` supports querying Postgis geometries as GeoJSON without explicitly querying them as such. It is necessary to pass in the `--postgis` flag in order to query Postgis types (as their oids are unknown at compile time).

```bash
casql query --conn local-postgis "SELECT * FROM places LIMIT 1"
# [{"point":{"type":"Point","crs":{"type":"name","properties":{"name":"EPSG:32632"}},"coordinates":[1.2,3.4]}}]
```

## Current Limitations
#### Security
Currently handles md5-hashed passwords, plaintext password and unauthenticated databases, but not other the other methods Postgres supports.

It doesn’t support SSL connections, so it should not be used on insecure networks.

#### Types
While it should cover most of the common user-types in Postgres, there may be some that are still missing. If you come across one, please raise an issue.

The Postgis support is in progress. Most GeoJSON types are done, but not every type that can be saved in a Geometry columns. 

#### Interface
The CLI interface is clunky, and may be subject to change.

### Local Testing
```bash
docker run --rm --name pg-test-db -p 5432:5432 \
-e POSTGRES_USER=root -e POSTGRES_PASSWORD=cascat -e POSTGRES_DB=dbname \
-v $(pwd)/tests/resources:/docker-entrypoint-initdb.d \
postgres:13
```

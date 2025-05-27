# PostGIS - Working With Geospatial Data in PostgreSQL

To start a PostgreSQL container run:

```
make start
```

To connect to the running container, open another terminal and use:

```
make connect
```

To connect to the database:

```
psql -U postgres
```

To create a table that stores places:

```sql
CREATE TABLE places (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64),
    location GEOMETRY(POINT, 4326)
);
```

To create an index in the location field:

```sql
CREATE INDEX idx_places_location ON places USING GIST (location);
```

To insert a point:

```sql
INSERT INTO places(name, location)
VALUES('Central Park', ST_GeomFromText('POINT(-73.9654 40.7829)'));
```

To query the point:

```sql
SELECT * FROM places;
SELECT id, name, ST_AsText(location) FROM places;
SELECT id, name, ST_AsGeojson(location) FROM places;
```

To create a table that stores polygons:

```sql
CREATE TABLE areas (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    boundary GEOMETRY(POLYGON, 4326)
);
```

Insert an area:

```sql
INSERT INTO areas (name, boundary)
VALUES (
    'Central Park',
    ST_GeomFromText('POLYGON((
        -73.9731 40.7644,
        -73.9819 40.7681,
        -73.9580 40.8006,
        -73.9498 40.7968,
        -73.9731 40.7644
    ))')
);
```

Query the area:

```sql
SELECT * FROM areas;
SELECT id, name, ST_AsText(boundary) FROM areas;
SELECT id, name, ST_AsGeojson(boundary) FROM areas;
```

To insert a Geojson:

```sql
INSERT INTO areas (name, boundary)
VALUES (
    'Dolores Park',
    ST_GeomFromGeoJSON('{
        "type": "Polygon",
        "coordinates": [
          [
            [ -122.42836549322851, 37.76123463001613 ],
            [ -122.42806139698865, 37.75811926165099 ],
            [ -122.42592638797277, 37.758254497406654 ],
            [ -122.42620514285912, 37.76137987710844 ],
            [ -122.42836549322851, 37.76123463001613 ]
          ]
        ]
    }')
);
```

Example of geo spatial queries:

```sql
SELECT ST_Contains(
  (SELECT boundary FROM areas WHERE name = 'Central Park' LIMIT 1),
  (SELECT location FROM places WHERE name = 'Central Park' LIMIT 1)
);
```

To get all places in Central Park:

```sql
SELECT id, name
FROM places
WHERE ST_Contains(
  ST_GeomFromText(
    'POLYGON((
      -73.9731 40.7644,
      -73.9819 40.7681,
      -73.9580 40.8006,
      -73.9498 40.7968,
      -73.9731 40.7644
    ))', 4326),
    location
);
```

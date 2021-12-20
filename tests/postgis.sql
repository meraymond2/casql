-- TODO: the postgis image doesn’t work if I pass in an init script. Maybe I just need to write a couple of bash scripts to start up the test images.

-- Postgis Points --
CREATE TABLE points
(
    point geometry
);

INSERT INTO points
VALUES ('POINT(1.2 3.4)'),
       ('POINT(1.2 3.4 5.6)'),
       ('POINTM(1.2 3.4 5.6)'),
       ('POINT(1.2 3.4 5.6 7.8)'),
       ('SRID=32632;POINT(1.2 3.4)'),
       ('SRID=32632;POINT(1.2 3.4 5.6)'),
       ('SRID=32632;POINT(1.2 3.4 5.6)'),
       ('SRID=32632;POINTM(1.2 3.4 5.6)'),
       ('SRID=4326;POINT(1.2 3.4 5.6 7.8)');

CREATE TABLE multipoints
(
    multipoint geometry
);

INSERT INTO multipoints
VALUES ('MULTIPOINT((13.595 56.429), (14.287 56.343), (14.150 56.731))');


CREATE TABLE lines
(
    line geometry
);

INSERT INTO lines
VALUES ('LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
       ('SRID=4326;LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
       ('LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('SRID=4326;LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('SRID=4326;LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('LINESTRING(1.2 3.4 5.6 7.8, 9.10 11.12 13.14 15.16, 17.18 19.20 21.22 23.24)');

CREATE TABLE multilines
(
    multiline geometry
);

INSERT INTO multilines
VALUES ('MULTILINESTRING((14.172 56.829, 14.243 57.087, 14.889 57.113), (14.331 56.354, 15.067 56.479, 15.185 56.745))');


CREATE TABLE polys
(
    poly geometry
);

INSERT INTO polys
VALUES ('POLYGON((10.607986450195312 59.80598737346893,10.705146789550781 59.799598075478414,10.654335021972656 59.87050238394518,10.607986450195312 59.80598737346893))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893,10.705146789550781 59.799598075478414,10.654335021972656 59.87050238394518,10.607986450195312 59.80598737346893))'),
       ('POLYGONM((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('SRID=4326;POLYGONM((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('POLYGON((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('POLYGON((10.607986450195312 59.80598737346893 5.0 1,10.705146789550781 59.799598075478414 5.0 2,10.654335021972656 59.87050238394518 5.0 3,10.607986450195312 59.80598737346893 5.0 4))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893 5.0 1,10.705146789550781 59.799598075478414 5.0 2,10.654335021972656 59.87050238394518 5.0 3,10.607986450195312 59.80598737346893 5.0 4))');

CREATE TABLE multipolys
(
    multipoly geometry
);

INSERT INTO multipolys
VALUES ('MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)),((20 35, 10 30, 10 10, 30 5, 45 20, 20 35),(30 20, 20 15, 20 25, 30 20)))');

CREATE TABLE geo_coll (
    coll geometry
);

INSERT INTO geo_coll VALUES ('SRID=32632;GEOMETRYCOLLECTION (POINT (40 10), MULTIPOINT ((40 10),(10 40)),LINESTRING (10 10, 20 20, 10 40),POLYGON ((40 40, 20 45, 45 30, 40 40)))');
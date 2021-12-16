-- Postgis Points --
-- CREATE TABLE points
-- (
--     point geometry
-- );
--
-- INSERT INTO points (point)
-- VALUES ('POINT(1.2 3.4)'),
--        ('POINT(1.2 3.4 5.6)'),
--        ('POINTM(1.2 3.4 5.6)'),
--        ('POINT(1.2 3.4 5.6 7.8)'),
--        ('SRID=32632;POINT(1.2 3.4)'),
--        ('SRID=32632;POINT(1.2 3.4 5.6)'),
--        ('SRID=32632;POINT(1.2 3.4 5.6)'),
--        ('SRID=32632;POINTM(1.2 3.4 5.6)'),
--        ('SRID=4326;POINT(1.2 3.4 5.6 7.8)');

-- CREATE TABLE lines
-- (
--     line geometry
-- );
--
-- INSERT INTO lines (line)
-- VALUES ('LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
--        ('SRID=4326;LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
--        ('LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('SRID=4326;LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('SRID=4326;LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('LINESTRING(1.2 3.4 5.6 7.8, 9.10 11.12 13.14 15.16, 17.18 19.20 21.22 23.24)');

-- CREATE TABLE lines
-- (
--     line geometry
-- );
--
-- INSERT INTO lines (line)
-- VALUES ('LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
--        ('SRID=4326;LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
--        ('LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('SRID=4326;LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('SRID=4326;LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
--        ('LINESTRING(1.2 3.4 5.6 7.8, 9.10 11.12 13.14 15.16, 17.18 19.20 21.22 23.24)');

CREATE TABLE polys
(
    poly geometry
);

INSERT INTO polys (poly)
VALUES ('POLYGON((10.607986450195312 59.80598737346893,10.705146789550781 59.799598075478414,10.654335021972656 59.87050238394518,10.607986450195312 59.80598737346893))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893,10.705146789550781 59.799598075478414,10.654335021972656 59.87050238394518,10.607986450195312 59.80598737346893))'),
       ('POLYGONM((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('SRID=4326;POLYGONM((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('POLYGON((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893 1,10.705146789550781 59.799598075478414 2,10.654335021972656 59.87050238394518 3,10.607986450195312 59.80598737346893 4))'),
       ('POLYGON((10.607986450195312 59.80598737346893 5.0 1,10.705146789550781 59.799598075478414 5.0 2,10.654335021972656 59.87050238394518 5.0 3,10.607986450195312 59.80598737346893 5.0 4))'),
       ('SRID=4326;POLYGON((10.607986450195312 59.80598737346893 5.0 1,10.705146789550781 59.799598075478414 5.0 2,10.654335021972656 59.87050238394518 5.0 3,10.607986450195312 59.80598737346893 5.0 4))');

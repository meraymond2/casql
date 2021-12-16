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

CREATE TABLE lines
(
    line geometry
);

INSERT INTO lines (line)
VALUES ('LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
       ('SRID=4326;LINESTRING(1.2 3.4,5.6 7.8,9.10 11.12)'),
       ('LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('SRID=4326;LINESTRINGM(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('SRID=4326;LINESTRING(1.2 3.4 5.6, 7.8 9.10 11.12, 13.14 15.16 17.18)'),
       ('LINESTRING(1.2 3.4 5.6 7.8, 9.10 11.12 13.14 15.16, 17.18 19.20 21.22 23.24)');

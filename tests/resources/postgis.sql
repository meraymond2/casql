-- Postgis Points --
CREATE TABLE points
(
    geom geometry
);

INSERT INTO points (geom)
VALUES ('POINT(1.2 3.4)'),
       ('POINT(1.2 3.4 5.6)'),
       ('POINTM(1.2 3.4 5.6)'),
       ('POINT(1.2 3.4 5.6 7.8)'),
       ('SRID=32632;POINT(1.2 3.4)'),
       ('SRID=32632;POINT(1.2 3.4 5.6)'),
       ('SRID=32632;POINT(1.2 3.4 5.6)'),
       ('SRID=32632;POINTM(1.2 3.4 5.6)'),
       ('SRID=4326;POINT(1.2 3.4 5.6 7.8)');
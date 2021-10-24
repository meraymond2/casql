# EWKB
https://postgis.net/docs/using_postgis_dbmanagement.html#EWKB_EWKT

WKT/WKB are specified by OpenGIS, and that version is slightly simpler than PostGIS's. They don't include an SRID, and are only 2d.

EWKB is PostGIS's extended WKB format. It's not very well documented, at least there's no official spec, but I think that's because it's basically WKB with some extra headers.

EWKB adds SRID info, and 3DM, 3DZ, 4D coordinate support.

3DM is 2d + a measurement field. (XYM)
3DZ is 3d coords. (XYZ)
4D is 3d + mesurement field (XYZM)

I think the SRID is separate, and can apply to any coord type.

There also appear to be more object types, like TRIANGLE?

## Parsing
So my hypothesis is that I just need to figure out how to parse the headers, and then that should tell me how many doubles it is per coord. And use that to parse coords.

## Questions
Do the M coords only apply to points? They don't have any examples of M-Lines. I think the measurement is supposed to apply to the geom, so that would be tricky. They give examples for m-point and m-multipoint. GeoJSON specifically warns against putting the M in the position array, so for a multipoint, I could provide an array of Ms in properties. It's pretty arbitrary, they don't correspond 1-1, so it's just best effort.

# GeoJSON
https://datatracker.ietf.org/doc/html/rfc7946

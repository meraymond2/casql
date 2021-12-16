-- Numbers --
CREATE TABLE integers
(
    bool bool, -- 16: boolean, \'true\'/\'false\'
    int2 int2, -- 21: -32 thousand to 32 thousand, 2-byte storage
    int4 int4, -- 23: -2 billion to 2 billion integer, 4-byte storage
    int8 int8  -- 20: ~18 digit integer, 8-byte storage
);

INSERT INTO integers (bool, int2, int4, int8)
VALUES (true, 12345, 12345678, 123456790123);

CREATE TABLE floats
(
    float4 float4, -- 700: single-precision floating point number, 4-byte storage
    float8 float8  -- 701: double-precision floating point number, 8-byte storage
);

INSERT INTO floats (float4, float8)
VALUES (3.1415926535897932384626433, 3.1415926535897932384626433),
       ('NaN', 'NaN'),
       ('Infinity', 'Infinity'),
       ('-Infinity', '-Infinity');

CREATE TABLE numerics
(
    numeric     numeric,       -- 1700: numeric(precision, decimal), arbitrary precision number
    zero_scale  numeric(10),   -- 10 digit precision, 0 scale
    fixed_scale numeric(10, 5) -- 10 digit precision, 5 digit scale
);

INSERT INTO numerics (numeric, zero_scale, fixed_scale)
VALUES (1234567, 12345.54321, 12345.54321),
       (1, 2, 3),
       (0.0000000002, 0.002, 0.000026),
       (-0.0000000002, -0.002, -0.000026),
       (100000000000, 1000000000, 10000),
       (NULL, NULL, NULL),
       ('NaN', 'NaN', 'NaN');

-- Text --
CREATE TABLE texts
(
    -- just char as a type is implied to be char(1), oid 1042, not 18
    char            pg_catalog.char, -- 18: single character
    fixed_char      char(3),         -- 1042: bpchar, char(length), blank-padded string, fixed storage length
    name            name,            -- 19: 63-byte type for storing system identifiers
    text            text,            -- 25: variable-length string, no limit specified
    varchar         varchar,         -- 1043: varchar(length), non-blank-padded string, variable storage length
    bounded_varchar varchar(9)
);

INSERT INTO texts (char, fixed_char, name, text, varchar, bounded_varchar)
VALUES ('O', 'wee', 'sleekrit', 'cowran', 'timrous', '"beastie"');

-- Binary --
CREATE TABLE binaries
(
    bytea          bytea,  -- 17: variable-length string, binary values escaped
    bit            bit,    -- 1560: fixed-length bit string
    octet          bit(8),
    varbit         varbit, -- 1562: variable-length bit string
    bounded_varbit varbit(64)
);

INSERT INTO binaries (bytea, bit, octet, varbit, bounded_varbit)
VALUES ('\x5468657265E2809973206E6F2073756368207468696E67206173203221', B'1', B'00001010', B'10101',
        B'0000000000000000000000000000000000000000000000000000000000001000');

-- Date/Time --
CREATE TABLE dates_and_times
(
    date        date,        -- 1082: date
    time        time,        -- 1083: time of day
    timestamp   timestamp,   -- 1114: date and time
    timestamptz timestamptz, -- 1184: date and time with time zone
    interval    interval,    -- 1186: @ <number> <units>, time interval
    timetz      timetz       -- 1266: time of day with time zone
);

INSERT INTO dates_and_times (date, time, timestamp, timestamptz, interval, timetz)
VALUES ('4713-01-1 BC', '04:05:06.789', '4713-01-1 04:05:06.789 BC', '4713-01-1 04:05:06.789 BC -0:00', '0',
        '04:05:06.789-8'),
       ('0002-12-31', 'allballs', '0002-12-31 00:00:00', '0002-12-31 00:00:00 CET',
        '1 year 2 months 3 days 4 hours 5 minutes 6 seconds ago',
        '04:05:06 PST'),
       ('2200-01-01', '04:05 PM', '2200-01-01 04:05 PM', '2200-01-01 04:05 PM PST',
        '-13 months, 12 days, -13 minutes, 12.12 seconds',
        '04:05:06+03:02:01'),
       ('5874897-12-31', '23:59:59.999999', '294276-12-31 23:59:59.999999', '294276-12-31 23:59:59.999999 z',
        '15 microseconds', '04:05:06.789Z');

/*

-- Internal Ids --
regproc regproc, -- 24: registered procedure
oid oid, -- 26: object identifier(oid), maximum 4 billion
tid tid, -- 27: (block, offset), physical location of tuple
xid xid, -- 28: transaction id
cid cid, -- 29: command identifier type, sequence in transaction id
oidvector oidvector, -- 30: array of oids, used in system tables
xid8 xid8, -- 5069: full transaction id

-- Structured Data --
json json, -- 114: JSON stored as text
xml xml, -- 142: XML content
uuid uuid, -- 2950: UUID datatype
jsonb jsonb, -- 3802: Binary JSON
jsonpath jsonpath, -- 4072: JSON path

-- Postgres Geometries --
point point, -- 600: geometric point \'(x, y)\'
lseg lseg, -- 601: geometric line segment \'(pt1,pt2)\'
path path, -- 602: geometric path \'(pt1,...)\'
box box, -- 603: geometric box \'(lower left,upper right)\'
polygon polygon, -- 604: geometric polygon \'(pt1,...)\'
line line, -- 628: geometric line
circle circle, -- 718: geometric circle \'(center,radius)\'

-- Networking --
macaddr macaddr, -- 829: XX:XX:XX:XX:XX:XX, MAC address
inet inet, -- 869: IP address/netmask, host address, netmask optional
cidr cidr, -- 650: network IP address/netmask, network address
macaddr8 macaddr8, -- 774: XX:XX:XX:XX:XX:XX:XX:XX, MAC address


int2vector int2vector, -- 22: array of int2, used in system tables
pg_type pg_type, -- 71:
pg_attribute pg_attribute, -- 75:
pg_proc pg_proc, -- 81:
pg_class pg_class, -- 83:
pg_node_tree pg_node_tree, -- 194: string representing an internal node tree
pg_ndistinct pg_ndistinct, -- 3361: multivariate ndistinct coefficients
pg_dependencies pg_dependencies, -- 3402: multivariate dependencies
pg_mcv_list pg_mcv_list, -- 5017: multivariate MCV list
pg_ddl_command pg_ddl_command, -- 32: internal type for passing CollectedCommand
unknown unknown, -- 705: pseudo-type representing an undetermined type
money money, -- 790: monetary amounts, $d,ddd.cc
aclitem aclitem, -- 1033: access control list

refcursor refcursor, -- 1790: reference to cursor (portal name)
regprocedure regprocedure, -- 2202: registered procedure (with args)
regoper regoper, -- 2203: registered operator
regoperator regoperator, -- 2204: registered operator (with args)
regclass regclass, -- 2205: registered class
regcollation regcollation, -- 4191: registered collation
regtype regtype, -- 2206: registered type
regrole regrole, -- 4096: registered role
regnamespace regnamespace, -- 4089: registered namespace
pg_lsn pg_lsn, -- 3220: PostgreSQL LSN datatype
tsvector tsvector, -- 3614: text representation for text search
gtsvector gtsvector, -- 3642: GiST index internal text representation for text search
tsquery tsquery, -- 3615: query representation for text search
regconfig regconfig, -- 3734: registered text search configuration
regdictionary regdictionary, -- 3769: registered text search dictionary
txid_snapshot txid_snapshot, -- 2970: txid snapshot
pg_snapshot pg_snapshot, -- 5038: snapshot
int4range int4range, -- 3904: range of integers
numrange numrange, -- 3906: range of numerics
tsrange tsrange, -- 3908: range of timestamps without time zone
tstzrange tstzrange, -- 3910: range of timestamps with time zone
daterange daterange, -- 3912: range of dates
int8range int8range, -- 3926: range of bigints
int4multirange int4multirange, -- 4451: multirange of integers
nummultirange nummultirange, -- 4532: multirange of numerics
tsmultirange tsmultirange, -- 4533: multirange of timestamps without time zone
tstzmultirange tstzmultirange, -- 4534: multirange of timestamps with time zone
datemultirange datemultirange, -- 4535: multirange of dates
int8multirange int8multirange, -- 4536: multirange of bigints
record record, -- 2249: pseudo-type representing any composite type
_record _record, -- 2287:
cstring cstring, -- 2275: C-style string
any any, -- 2276: pseudo-type representing any type
anyarray anyarray, -- 2277: pseudo-type representing a polymorphic array type
void void, -- 2278: pseudo-type for the result of a function with no real result
trigger trigger, -- 2279: pseudo-type for the result of a trigger function
event_trigger event_trigger, -- 3838: pseudo-type for the result of an event trigger function
language_handler language_handler, -- 2280: pseudo-type for the result of a language handler function
internal internal, -- 2281: pseudo-type representing an internal data structure
anyelement anyelement, -- 2283: pseudo-type representing a polymorphic base type
anynonarray anynonarray, -- 2776: pseudo-type representing a polymorphic base type that is not an array
anyenum anyenum, -- 3500: pseudo-type representing a polymorphic base type that is an enum
fdw_handler fdw_handler, -- 3115: pseudo-type for the result of an FDW handler function
index_am_handler index_am_handler, -- 325: pseudo-type for the result of an index AM handler function
tsm_handler tsm_handler, -- 3310: pseudo-type for the result of a tablesample method function
table_am_handler table_am_handler, -- 269:
anyrange anyrange, -- 3831: pseudo-type representing a range over a polymorphic base type
anycompatible anycompatible, -- 5077: pseudo-type representing a polymorphic common type
anycompatiblearray anycompatiblearray, -- 5078: pseudo-type representing an array of polymorphic common type elements
anycompatiblenonarray anycompatiblenonarray, -- 5079: pseudo-type representing a polymorphic common type that is not an array
anycompatiblerange anycompatiblerange, -- 5080: pseudo-type representing a range over a polymorphic common type
anymultirange anymultirange, -- 4537: pseudo-type representing a polymorphic base type that is a multirange
anycompatiblemultirange anycompatiblemultirange, -- 4538: pseudo-type representing a multirange over a polymorphic common type
pg_brin_bloom_summary pg_brin_bloom_summary, -- 4600: BRIN bloom summary
pg_brin_minmax_multi_summary pg_brin_minmax_multi_summary, -- 4601: BRIN minmax-multi summary

bool[] -- 1000: array of 16
bytea[] -- 1001: array of 17
char[] -- 1002: array of 18
name[] -- 1003: array of 19
int8[] -- 1016: array of 20
int2[] -- 1005: array of 21
int2vector[] -- 1006: array of 22
int4[] -- 1007: array of 23
regproc[] -- 1008: array of 24
text[] -- 1009: array of 25
oid[] -- 1028: array of 26
tid[] -- 1010: array of 27
xid[] -- 1011: array of 28
cid[] -- 1012: array of 29
oidvector[] -- 1013: array of 30
pg_type[] -- 210: array of 71
pg_attribute[] -- 270: array of 75
pg_proc[] -- 272: array of 81
pg_class[] -- 273: array of 83
json[] -- 199: array of 114
xml[] -- 143: array of 142
xid8[] -- 271: array of 5069
point[] -- 1017: array of 600
lseg[] -- 1018: array of 601
path[] -- 1019: array of 602
box[] -- 1020: array of 603
polygon[] -- 1027: array of 604
line[] -- 629: array of 628
float4[] -- 1021: array of 700
float8[] -- 1022: array of 701
circle[] -- 719: array of 718
money[] -- 791: array of 790
macaddr[] -- 1040: array of 829
inet[] -- 1041: array of 869
cidr[] -- 651: array of 650
macaddr8[] -- 775: array of 774
aclitem[] -- 1034: array of 1033
bpchar[] -- 1014: array of 1042
varchar[] -- 1015: array of 1043
date[] -- 1182: array of 1082
time[] -- 1183: array of 1083
timestamp[] -- 1115: array of 1114
timestamptz[] -- 1185: array of 1184
interval[] -- 1187: array of 1186
timetz[] -- 1270: array of 1266
bit[] -- 1561: array of 1560
varbit[] -- 1563: array of 1562
numeric[] -- 1231: array of 1700
refcursor[] -- 2201: array of 1790
regprocedure[] -- 2207: array of 2202
regoper[] -- 2208: array of 2203
regoperator[] -- 2209: array of 2204
regclass[] -- 2210: array of 2205
regcollation[] -- 4192: array of 4191
regtype[] -- 2211: array of 2206
regrole[] -- 4097: array of 4096
regnamespace[] -- 4090: array of 4089
uuid[] -- 2951: array of 2950
pg_lsn[] -- 3221: array of 3220
tsvector[] -- 3643: array of 3614
gtsvector[] -- 3644: array of 3642
tsquery[] -- 3645: array of 3615
regconfig[] -- 3735: array of 3734
regdictionary[] -- 3770: array of 3769
jsonb[] -- 3807: array of 3802
jsonpath[] -- 4073: array of 4072
txid_snapshot[] -- 2949: array of 2970
pg_snapshot[] -- 5039: array of 5038
int4range[] -- 3905: array of 3904
numrange[] -- 3907: array of 3906
tsrange[] -- 3909: array of 3908
tstzrange[] -- 3911: array of 3910
daterange[] -- 3913: array of 3912
int8range[] -- 3927: array of 3926
int4multirange[] -- 6150: array of 4451
nummultirange[] -- 6151: array of 4532
tsmultirange[] -- 6152: array of 4533
tstzmultirange[] -- 6153: array of 4534
datemultirange[] -- 6155: array of 4535
int8multirange[] -- 6157: array of 4536
cstring[] -- 1263: array of 2275
*/

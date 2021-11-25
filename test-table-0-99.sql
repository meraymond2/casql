CREATE TABLE types_0_99
(
    bool    bool,    -- 16: boolean, \'true\'/\'false\'
    bytea   bytea,   -- 17: variable-length string, binary values escaped
    char    char,    -- 18: single character
    name    name,    -- 19: 63-byte type for storing system identifiers
    int8    int8,    -- 20: ~18 digit integer, 8-byte storage
    int2    int2,    -- 21: -32 thousand to 32 thousand, 2-byte storage
--     int2vector int2vector, -- 22: array of int2, used in system tables — not sure about literal
    int4    int4,    -- 23: -2 billion to 2 billion integer, 4-byte storage
    regproc regproc, -- 24: registered procedure
    text    text,    -- 25: variable-length string, no limit specified
    oid     oid,     -- 26: object identifier(oid), maximum 4 billion
    tid     tid,     -- 27: (block, offset), physical location of tuple
    xid     xid,     -- 28: transaction id
    cid     cid     -- 29: command identifier type, sequence in transaction id
--     oidvector  oidvector,  -- 30: array of oids, used in system tables — not sure about literal
--     pg_type    pg_type,    -- 71: — not sure about literal
--     pg_attribute pg_attribute, -- 75: ERROR:  column "attmissingval" has pseudo-type anyarray
--     pg_proc    pg_proc,    -- 81: — not sure about literal
--     pg_class   pg_class    -- 83: — not sure about literal
);

INSERT INTO types_0_99 (bool, bytea, char, name, int8, int2, int4, regproc, text, oid, tid, xid, cid)
VALUES (true,
        E'\\000001002003004005', -- octets for 0, 1, 2, 3, 4, 5
        'A',
        'name is michael',
        1234567890,
        12345,
        1234567890,
        77,
        'I’m a Postgres text value, how do you like me so far?', 77, '(0, 1)'::tid, '42'::xid, '34'::cid);

-- psql output
-- bool    t
-- bytea   \x00303031303032303033303034303035
-- char    A
-- name    name is michael
-- int8    1234567890
-- int2    12345
-- int4    1234567890
-- regproc pg_catalog.int4
-- text    I’m a Postgres text value, how do you like me so far?
-- oid     77
-- tid     (0,1)
-- xid     42
-- cid     34
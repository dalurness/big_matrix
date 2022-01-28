-- name: initialize_db
-- initializes the db to store map objects
CREATE TABLE cell (
    x       INTEGER NOT NULL,
    y       INTEGER NOT NULL,
    data    BLOB,
    PRIMARY KEY (x, y)
);
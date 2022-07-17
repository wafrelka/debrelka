CREATE TABLE IF NOT EXISTS "records" (
    timestamp INTEGER PRIMARY KEY,
    value REAL
);

INSERT INTO "records" (timestamp, value)
SELECT strftime('%s'), (value + (ABS(RANDOM()) % 20) / 10.0 - 1.0)
FROM "records" ORDER BY timestamp DESC LIMIT 1;

INSERT INTO "records" (timestamp, value)
SELECT strftime('%s'), 0.0
WHERE NOT EXISTS (SELECT * FROM "records");

SELECT * FROM "records" ORDER BY timestamp;

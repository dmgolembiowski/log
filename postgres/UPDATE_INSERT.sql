CREATE OR REPLACE FUNCTION upsert_ledger(LINEARG TEXT, LINENUMBERARG INTEGER) RETURNS VOID AS $$
    DECLARE
    BEGIN
        UPDATE $1.$1 SET (line, line_number) = (LINEARG, LINENUMBERARG) WHERE line_number = LINENUMBERARG;
        IF NOT FOUND THEN
        INSERT INTO $1.$1 (line, line_number) VALUES (LINEARG, LINENUMBERARG);
        END IF;
    END; $$
LANGUAGE PLPGSQL;
SELECT upsert_ledger($2, $3);

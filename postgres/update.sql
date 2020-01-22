BEGIN; 
    CREATE TABLE IF NOT EXISTS $1 (line text, line_number integer); 
    
    CREATE OR REPLACE 
        FUNCTION upsert_ledger(LINEARG TEXT, LINENUMBERARG INTEGER) RETURNS VOID AS $$ 
        DECLARE
            BEGIN
            UPDATE $1 SET (line, line_number) = (LINEARG, LINENUMBERARG)
                WHERE line_number = LINENUMBERARG;
                IF NOT FOUND THEN
                    INSERT INTO $1 (line, line_number) VALUES (LINEARG, LINENUMBERARG);
                END IF;
            END; $$
        LANGUAGE PLPGSQL; 
    
    SAVEPOINT gen_upsert_ledger;

    is_null := SELECT NULLIF($2, 'VOID');
    CASE WHEN is_null='(none)' THEN SELECT '(none)'
         ELSE SELECT upsert_ledger($2, $3);
    END
    SAVEPOINT populated_new_record; 
COMMIT; 
SELECT * FROM $1;

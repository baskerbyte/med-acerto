DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 100000 LOOP
        INSERT INTO comments (user_id, question_id, content)
        VALUES (
            (RANDOM() * 99999) + 1,
            (RANDOM() * 999999) + 1,
            'Fake comment ' || i
        );
        i := i + 1;
    END LOOP;
END$$;

DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 100000 LOOP
        DECLARE
            random_user_id INT := (RANDOM() * 99999) + 1;
            random_comment_id INT := (RANDOM() * 99999) + 1;
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM comment_likes WHERE user_id = random_user_id AND comment_id = random_comment_id) THEN
                INSERT INTO comment_likes (user_id, comment_id)
                VALUES (random_user_id, random_comment_id);
                i := i + 1;
            END IF;
        END;
    END LOOP;
END$$;
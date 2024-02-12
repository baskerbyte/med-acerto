DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 1000000 LOOP
        INSERT INTO questions (content, options, answer_idx, tag, year, origin, difficulty_rating)
        VALUES (
            'Fake question ' || i,
            ARRAY['Option A', 'Option B', 'Option C', 'Option D'],
            (i % 4) + 1,
            (i % 5) + 1,
            (i % 5) + 2017,
            (i % 3) + 1,
            RANDOM() * 5 + 1
        );
        i := i + 1;
    END LOOP;
END$$;

DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 100000 LOOP
        INSERT INTO answers (user_id, question_id, correct, answer_idx)
        VALUES (
            (RANDOM() * 99999) + 1,
            (RANDOM() * 999999) + 1,
            CASE WHEN (RANDOM() * 2)::INTEGER = 1 THEN TRUE ELSE FALSE END,
            (RANDOM() * 4) + 1
        );
        i := i + 1;
    END LOOP;
END$$;
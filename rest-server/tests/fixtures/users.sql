DO $$
DECLARE
    i INT := 1;
BEGIN
    WHILE i <= 100000 LOOP
        INSERT INTO users (name, username, email)
        VALUES (
            'User ' || i,
            'user' || i,
            'user' || i || '@example.com'
        );
        i := i + 1;
    END LOOP;
END $$;

DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 100000 LOOP
        INSERT INTO payments (user_id, method, status)
        VALUES (
            (RANDOM() * 99999) + 1,
            (RANDOM() * 3) + 1,
            (RANDOM() * 3) + 1
        );
        i := i + 1;
    END LOOP;
END$$;

DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 1000 LOOP
        INSERT INTO subscriptions (user_id, plan_type, payment_id, end_date, status)
        VALUES (
            (RANDOM() * 99999) + 1,
            (RANDOM() * 2) + 1,
            (RANDOM() * 99999) + 1,
            CURRENT_TIMESTAMP + INTERVAL '30 days',
            (RANDOM() * 2) + 1
        );
        i := i + 1;
    END LOOP;
END$$;
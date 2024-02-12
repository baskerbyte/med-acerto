CREATE TABLE IF NOT EXISTS "questions" (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    options TEXT[] NOT NULL,
    answer_idx SMALLINT NOT NULL,
    tag SMALLINT NOT NULL,
    year SMALLINT NOT NULL,
    origin SMALLINT NOT NULL,
    difficulty_rating FLOAT NOT NULL DEFAULT 1.0
);

CREATE INDEX question_tag ON questions (tag);
CREATE INDEX question_year ON questions (year);
CREATE INDEX question_origin ON questions (origin);
CREATE INDEX question_difficulty ON questions (difficulty_rating);

CREATE TABLE IF NOT EXISTS "answers" (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    question_id INT REFERENCES questions(id) NOT NULL,
    correct BOOLEAN NOT NULL,
    answer_idx SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX user_answer ON "answers" (user_id);
CREATE INDEX answer_question ON "answers" (question_id);
CREATE INDEX correct_answer ON "answers" (correct);
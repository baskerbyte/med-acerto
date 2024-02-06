CREATE TABLE IF NOT EXISTS "comments" (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    question_id INT REFERENCES questions(id) NOT NULL,
    content TEXT NOT NULL
);

CREATE INDEX question_comment ON "comments" (question_id);

CREATE TABLE IF NOT EXISTS "comment_likes" (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    comment_id INT REFERENCES comments(id) NOT NULL,
    UNIQUE (user_id, comment_id)
);

CREATE INDEX comment_like ON "comment_likes" (comment_id);
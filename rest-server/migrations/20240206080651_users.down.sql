DROP INDEX IF EXISTS user_email;
DROP INDEX IF EXISTS user_username;
DROP INDEX IF EXISTS user_subscription;

DROP TABLE IF EXISTS "users";
DROP TABLE IF EXISTS "payments";
DROP TABLE IF EXISTS "subscriptions";

DROP EXTENSION IF EXISTS "uuid-ossp";
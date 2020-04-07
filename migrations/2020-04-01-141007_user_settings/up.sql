CREATE TABLE user_settings (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  s_key TEXT NOT NULL,
  s_value TEXT NOT NULL
);
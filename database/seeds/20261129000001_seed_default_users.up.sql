-- Default accounts for a fresh deployment.
--
-- All three share the password "@PastiPAS123". Only the bcrypt hashes are stored,
-- and each row uses an independent salt. Change these credentials immediately
-- after deployment.

BEGIN;

WITH seeded_user AS (
  INSERT INTO users (role_id, name, bio, is_active)
  VALUES (
    (SELECT id FROM roles WHERE slug = 'super'),
    'Super Administrator',
    'Default deployment super administrator. Change the credentials after deploy.',
    TRUE
  )
  RETURNING id
)
INSERT INTO base_auths (user_id, username, password_hash)
SELECT id, 'super', '$2b$12$DI6eUWCPzenGOKD/U5dmQOUyHin9mePE6YnT4MqdyY2gUVErXz53W'
FROM seeded_user;

WITH seeded_user AS (
  INSERT INTO users (role_id, name, bio, is_active)
  VALUES (
    (SELECT id FROM roles WHERE slug = 'admin'),
    'Default Administrator',
    'Default deployment administrator. Change the credentials after deploy.',
    TRUE
  )
  RETURNING id
)
INSERT INTO base_auths (user_id, username, password_hash)
SELECT id, 'admin', '$2b$12$jMgNYZn8njiln5VuUGWvpuQi5ucYbxxaLaEUL0vb9Uv6KWB3J80Yi'
FROM seeded_user;

WITH seeded_user AS (
  INSERT INTO users (role_id, name, bio, is_active)
  VALUES (
    (SELECT id FROM roles WHERE slug = 'user'),
    'Default User',
    'Default deployment user. Change the credentials after deploy.',
    TRUE
  )
  RETURNING id
)
INSERT INTO base_auths (user_id, username, password_hash)
SELECT id, 'user', '$2b$12$H2QibO.YoZDIlt6R.QlVqeidHNZGiVRlS46cXqMD.H0fGxFtn5XMe'
FROM seeded_user;

COMMIT;

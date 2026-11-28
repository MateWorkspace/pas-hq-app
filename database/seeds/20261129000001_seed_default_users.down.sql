BEGIN;

WITH removed_credentials AS (
  DELETE FROM base_auths
  WHERE username IN ('super', 'admin', 'user')
  RETURNING user_id
)
DELETE FROM users
WHERE id IN (SELECT user_id FROM removed_credentials);

COMMIT;

BEGIN;

DELETE FROM role_permission
WHERE role_id IN (SELECT id FROM roles WHERE slug IN ('super', 'admin', 'user'));

DELETE FROM permissions
WHERE slug IN (
  'role:create', 'role:update', 'role:delete', 'role:list', 'role:grant',
  'permission:create', 'permission:list',
  'user:create', 'user:update', 'user:delete', 'user:list', 'user:get',
  'product:create', 'product:update', 'product:delete', 'product:list', 'product:get',
  'variant:manage', 'selection:manage',
  'sku:update', 'sku:delete', 'sku:regenerate', 'sku:list',
  'price:set', 'price:remove', 'price:list',
  'stock:adjust', 'stock:list', 'movement:transfer', 'movement:list',
  'apikey:list', 'apikey:create', 'apikey:rotate', 'apikey:update', 'apikey:delete',
  'location:create', 'location:update', 'location:delete', 'location:list', 'location:get'
);

DELETE FROM roles WHERE slug IN ('super', 'admin', 'user');

COMMIT;

db = db.getSiblingDB('yalc_db');

db.createUser({
  user: 'yalc_app',
  pwd: 'secure_app_password',
  roles: [
    {
      role: 'readWrite',
      db: 'yalc_db'
    }
  ]
});

print('MongoDB Least Privilege App User Created.');

db = db.getSiblingDB('ferrox_db');

db.createUser({
  user: 'ferrox_app',
  pwd: 'secure_app_password',
  roles: [
    {
      role: 'readWrite',
      db: 'ferrox_db'
    }
  ]
});

print('MongoDB Least Privilege App User Created.');

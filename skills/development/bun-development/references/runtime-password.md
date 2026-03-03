# Password Hashing

Bun provides secure password hashing with `Bun.password` using Argon2.

## Hashing Passwords

```typescript
// Hash a password (uses Argon2id by default)
const hash = await Bun.password.hash("user-password-123");

// Store hash in database
await db.run(
  "INSERT INTO users (email, password_hash) VALUES (?, ?)",
  [email, hash]
);
```

## Verifying Passwords

```typescript
// Retrieve stored hash from database
const user = await db.query("SELECT password_hash FROM users WHERE email = ?")
  .get(email);

// Verify password
const isValid = await Bun.password.verify("user-password-123", user.password_hash);

if (isValid) {
  // Password matches - login success
} else {
  // Password doesn't match - login failed
}
```

## Algorithm Details

Bun.password uses Argon2id by default:

```typescript
const hash = await Bun.password.hash("password", {
  algorithm: "argon2id",  // default
  memoryCost: 19456,      // memory in KB
  timeCost: 2,            // iterations
});
```

Supported algorithms:
- `argon2id` (default, recommended)
- `argon2i`
- `argon2d`
- `bcrypt`

## Best Practices

1. **Always hash passwords** - never store plaintext
2. **Use the defaults** - Bun's defaults are secure
3. **Verify with timing-safe comparison** - Bun.password.verify is timing-safe
4. **Don't log hashes** - treat like passwords
5. **Rate-limit login attempts** - prevent brute force

## Complete Authentication Example

```typescript
import { Database } from "bun:sqlite";

const db = new Database("users.db");

// Create users table
db.run(`
  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
  )
`);

// Register user
async function registerUser(email: string, password: string) {
  const hash = await Bun.password.hash(password);
  
  try {
    db.run(
      "INSERT INTO users (email, password_hash) VALUES (?, ?)",
      [email, hash]
    );
    return { success: true };
  } catch (error) {
    return { success: false, error: "Email already exists" };
  }
}

// Login user
async function loginUser(email: string, password: string) {
  const user = db.query("SELECT * FROM users WHERE email = ?").get(email);
  
  if (!user) {
    return { success: false, error: "Invalid credentials" };
  }
  
  const isValid = await Bun.password.verify(password, user.password_hash);
  
  if (isValid) {
    return { success: true, userId: user.id };
  } else {
    return { success: false, error: "Invalid credentials" };
  }
}

// Usage
await registerUser("user@example.com", "securePassword123");
const result = await loginUser("user@example.com", "securePassword123");
console.log(result);  // { success: true, userId: 1 }
```

## Security Audit

Check for plaintext password storage:

```bash
# Find potential plaintext password storage
rg "password\s*=.*req\.(body|json)" --type ts

# Find missing password hashing
rg "INSERT.*password.*VALUES" --type ts

# Ensure Bun.password.hash is used
rg "Bun\.password\.hash" --type ts
```

## Anti-patterns

- **DON'T** store passwords in plaintext
- **DON'T** use weak hashing (MD5, SHA1)
- **DON'T** implement your own crypto
- **DON'T** expose password hashes in logs or API responses
- **DON'T** skip rate limiting on login endpoints

## Migration from bcrypt

```typescript
// Old bcrypt code
import bcrypt from "bcrypt";
const hash = await bcrypt.hash(password, 10);
const isValid = await bcrypt.compare(password, hash);

// New Bun.password code
const hash = await Bun.password.hash(password);
const isValid = await Bun.password.verify(password, hash);
```

## References

- https://bun.sh/docs/api/hashing#bun-password
- https://en.wikipedia.org/wiki/Argon2

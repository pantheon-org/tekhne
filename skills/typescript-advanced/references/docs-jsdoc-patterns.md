# JSDoc Best Practices and Patterns

JSDoc comments provide runtime documentation for TypeScript code. They appear in IDE tooltips, generated documentation, and help other developers understand your code.

## When to Write JSDoc

**Always document:**
- Public API functions, classes, interfaces
- Complex algorithms or business logic
- Non-obvious behavior or side effects
- Security-sensitive code
- Deprecated APIs

**Don't document:**
- Self-explanatory code where the type signature is sufficient
- Private implementation details
- Obvious getters/setters
- Simple utility functions with clear names

## Basic Structure

```typescript
/**
 * Brief one-line summary
 *
 * Optional longer description with multiple paragraphs.
 * Can include markdown formatting.
 *
 * @param paramName - Description of parameter
 * @param anotherParam - Another parameter description
 * @returns Description of return value
 * @throws {ErrorType} Description of when this error is thrown
 *
 * @example
 * ```typescript
 * const result = functionName('value');
 * ```
 */
function functionName(paramName: string, anotherParam: number): ResultType {
  // implementation
}
```

## Documenting Functions

```typescript
/**
 * Authenticates a user and returns access tokens
 *
 * @param email - User's email address
 * @param password - User's password (will be hashed)
 * @returns Authentication result with access and refresh tokens
 * @throws {InvalidCredentialsError} If email or password is incorrect
 * @throws {AccountLockedError} If account has been locked due to failed attempts
 *
 * @example
 * ```typescript
 * const result = await authenticateUser('user@example.com', 'password123');
 * console.log(result.accessToken);
 * ```
 */
async function authenticateUser(
  email: string,
  password: string
): Promise<AuthResult> {
  // implementation
}
```

## Documenting Classes

```typescript
/**
 * Service for managing user authentication and authorization
 *
 * @remarks
 * This service handles JWT-based authentication, password hashing,
 * and role-based access control. All passwords are hashed with bcrypt
 * using a cost factor of 12.
 *
 * @example
 * ```typescript
 * const authService = new AuthService(config);
 * const token = await authService.login(email, password);
 * const user = await authService.verifyToken(token);
 * ```
 */
export class AuthService {
  /**
   * Creates a new authentication service
   *
   * @param config - Service configuration including JWT secret and token expiry
   */
  constructor(private config: AuthConfig) {}

  /**
   * Authenticates a user and returns access tokens
   *
   * @param credentials - User login credentials
   * @returns Authentication result with tokens
   * @throws {InvalidCredentialsError} If credentials are invalid
   */
  async login(credentials: LoginCredentials): Promise<AuthResult> {
    // implementation
  }
}
```

## Documenting Interfaces and Types

```typescript
/**
 * Represents a user in the system
 *
 * @property id - Unique user identifier (UUID v4)
 * @property email - User's email address (must be verified)
 * @property role - User's role for access control
 * @property createdAt - When the user account was created
 */
interface User {
  id: string;
  email: string;
  role: 'admin' | 'user' | 'guest';
  createdAt: Date;
}

/**
 * Configuration options for the authentication service
 */
interface AuthConfig {
  /**
   * Secret key for signing JWT tokens
   * @remarks Should be at least 32 characters and stored securely
   */
  jwtSecret: string;

  /**
   * How long access tokens are valid (in seconds)
   * @default 3600
   */
  tokenExpiry?: number;
}
```

## Documenting Generic Types

```typescript
/**
 * Wraps a value with loading and error states
 *
 * @template T - The type of the wrapped data
 *
 * @example
 * ```typescript
 * type UserAsync = Async<User>;
 * const loading: UserAsync = { status: 'loading' };
 * const success: UserAsync = { status: 'success', data: user };
 * ```
 */
type Async<T> =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: Error };
```

## Common JSDoc Tags

| Tag | Purpose | Example |
|-----|---------|---------|
| `@param` | Document function parameter | `@param userId - The user's ID` |
| `@returns` | Document return value | `@returns The user object` |
| `@throws` | Document exceptions thrown | `@throws {NotFoundError} If user not found` |
| `@example` | Provide usage example | `@example const x = foo('bar');` |
| `@remarks` | Additional detailed info | `@remarks This function is rate limited` |
| `@see` | Link to related documentation | `@see {@link AuthService}` |
| `@deprecated` | Mark as deprecated | `@deprecated Use newFunction instead` |
| `@default` | Document default value | `@default 3600` |
| `@template` | Document generic type parameter | `@template T - The item type` |
| `@internal` | Mark as internal API | `@internal` |
| `@beta` | Mark as beta/unstable | `@beta` |

## Linking to Other Symbols

```typescript
/**
 * Retrieves a user by ID
 *
 * @param userId - The user ID to look up
 * @returns The user object
 * @throws {NotFoundError} If user doesn't exist
 *
 * @see {@link User} for the user type definition
 * @see {@link createUser} for creating new users
 */
function getUser(userId: string): Promise<User> {
  // implementation
}
```

## Documenting Complex Examples

```typescript
/**
 * Processes a batch of user updates with transaction support
 *
 * @param updates - Array of user updates to process
 * @returns Results array with success/failure for each update
 *
 * @example Basic usage
 * ```typescript
 * const updates = [
 *   { id: '1', name: 'Alice' },
 *   { id: '2', email: 'bob@example.com' }
 * ];
 * const results = await processBatchUpdates(updates);
 * ```
 *
 * @example With error handling
 * ```typescript
 * try {
 *   const results = await processBatchUpdates(updates);
 *   const failures = results.filter(r => !r.success);
 *   if (failures.length > 0) {
 *     console.error('Some updates failed:', failures);
 *   }
 * } catch (error) {
 *   console.error('Batch update failed:', error);
 * }
 * ```
 */
async function processBatchUpdates(
  updates: UpdateUser[]
): Promise<BatchResult[]> {
  // implementation
}
```

## Deprecation Notices

```typescript
/**
 * @deprecated Since v2.0.0. Use {@link authenticateUser} instead.
 *
 * This function will be removed in v3.0.0.
 *
 * Migration guide:
 * ```typescript
 * // Old
 * const user = await login(email, password);
 *
 * // New
 * const result = await authenticateUser(email, password);
 * const user = result.user;
 * ```
 */
function login(email: string, password: string): Promise<User> {
  // implementation
}
```

## Security Documentation

```typescript
/**
 * Stores a user's password hash in the database
 *
 * @param userId - The user's ID
 * @param password - Plain text password (will be hashed before storage)
 *
 * @security
 * - Password is hashed using bcrypt with cost factor 12
 * - Original password is never stored
 * - Function should only be called over secure connections
 *
 * @remarks
 * This function should be called during user registration and password reset flows.
 * It automatically salts the password.
 */
async function storePassword(userId: string, password: string): Promise<void> {
  // implementation
}
```

## ESLint Integration

Enforce JSDoc with ESLint:

```json
{
  "rules": {
    "jsdoc/require-jsdoc": ["warn", {
      "require": {
        "FunctionDeclaration": true,
        "ClassDeclaration": true,
        "MethodDefinition": true
      }
    }],
    "jsdoc/require-param": "warn",
    "jsdoc/require-returns": "warn",
    "jsdoc/require-description": "warn"
  }
}
```

## References

- [TypeScript JSDoc Reference](https://www.typescriptlang.org/docs/handbook/jsdoc-supported-types.html)
- [JSDoc Official Documentation](https://jsdoc.app/)
- [eslint-plugin-jsdoc](https://github.com/gajus/eslint-plugin-jsdoc)

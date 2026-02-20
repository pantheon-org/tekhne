# API Documentation

## REST API Documentation

### Endpoint Structure

````markdown
## GET /api/users

Retrieves a list of users.

### Authentication

Requires Bearer token authentication.

### Request

```http
GET /api/users?page=1&limit=10 HTTP/1.1
Host: api.example.com
Authorization: Bearer {token}
```

### Query Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `page` | integer | No | 1 | Page number |
| `limit` | integer | No | 20 | Items per page (max 100) |
| `sort` | string | No | `created_at` | Sort field |
| `order` | string | No | `desc` | Sort order (`asc` or `desc`) |

### Response

**Success (200 OK)**

```json
{
  "data": [
    {
      "id": "user_123",
      "email": "user@example.com",
      "name": "John Doe",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 100,
    "pages": 5
  }
}
```

**Error (401 Unauthorized)**

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or expired token"
  }
}
```

### Example

```bash
curl -X GET "https://api.example.com/api/users?page=1&limit=10" \
  -H "Authorization: Bearer your-token-here"
```

```javascript
const response = await fetch('https://api.example.com/api/users', {
  headers: {
    'Authorization': `Bearer ${token}`
  }
});
const data = await response.json();
```
````

### POST Endpoint

````markdown
## POST /api/users

Creates a new user.

### Authentication

Requires Bearer token with `users:write` scope.

### Request

```http
POST /api/users HTTP/1.1
Host: api.example.com
Authorization: Bearer {token}
Content-Type: application/json

{
  "email": "newuser@example.com",
  "name": "Jane Doe",
  "role": "user"
}
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | string | Yes | User's email address (must be valid email) |
| `name` | string | Yes | User's full name (2-100 characters) |
| `role` | string | No | User role: `user`, `admin` (default: `user`) |

### Response

**Success (201 Created)**

```json
{
  "data": {
    "id": "user_456",
    "email": "newuser@example.com",
    "name": "Jane Doe",
    "role": "user",
    "created_at": "2024-01-16T14:20:00Z"
  }
}
```

**Error (400 Bad Request)**

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid request data",
    "details": [
      {
        "field": "email",
        "message": "Email already exists"
      }
    ]
  }
}
```

### Example

```bash
curl -X POST "https://api.example.com/api/users" \
  -H "Authorization: Bearer your-token-here" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@example.com",
    "name": "Jane Doe"
  }'
```
````

## Function Documentation

### JavaScript/TypeScript

````markdown
## fetchUser()

Fetches user data by ID.

### Signature

```typescript
function fetchUser(
  userId: string,
  options?: FetchOptions
): Promise<User>
```

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `userId` | `string` | Yes | The user's unique identifier |
| `options` | `FetchOptions` | No | Optional configuration |
| `options.timeout` | `number` | No | Request timeout in milliseconds (default: 5000) |
| `options.cache` | `boolean` | No | Enable caching (default: true) |

### Returns

`Promise<User>` - Resolves with user object containing:
- `id` (string): User ID
- `email` (string): User email
- `name` (string): User name
- `created_at` (string): ISO 8601 timestamp

### Throws

- `NotFoundError` - If user doesn't exist
- `TimeoutError` - If request times out
- `AuthError` - If authentication fails

### Example

```javascript
try {
  const user = await fetchUser('user_123', {
    timeout: 3000,
    cache: false
  });
  console.log(user.name);
} catch (error) {
  if (error instanceof NotFoundError) {
    console.error('User not found');
  }
}
```

### See Also

- [`fetchUsers()`](#fetchusers) - Fetch multiple users
- [`updateUser()`](#updateuser) - Update user data
````

### Python

````markdown
## fetch_user()

Fetches user data by ID.

### Signature

```python
def fetch_user(
    user_id: str,
    timeout: int = 5000,
    cache: bool = True
) -> User:
```

### Parameters

- **user_id** (`str`): The user's unique identifier
- **timeout** (`int`, optional): Request timeout in milliseconds. Default: `5000`
- **cache** (`bool`, optional): Enable caching. Default: `True`

### Returns

`User`: User object with the following attributes:
- `id` (str): User ID
- `email` (str): User email
- `name` (str): User name
- `created_at` (datetime): Creation timestamp

### Raises

- `NotFoundError`: If user doesn't exist
- `TimeoutError`: If request times out
- `AuthError`: If authentication fails

### Example

```python
try:
    user = fetch_user('user_123', timeout=3000, cache=False)
    print(user.name)
except NotFoundError:
    print('User not found')
```
````

## GraphQL Documentation

````markdown
## Query: user

Fetches a single user by ID.

### Schema

```graphql
type Query {
  user(id: ID!): User
}

type User {
  id: ID!
  email: String!
  name: String!
  posts: [Post!]!
  createdAt: DateTime!
}
```

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `id` | `ID!` | Yes | User's unique identifier |

### Example

```graphql
query GetUser {
  user(id: "user_123") {
    id
    email
    name
    posts {
      id
      title
    }
  }
}
```

### Response

```json
{
  "data": {
    "user": {
      "id": "user_123",
      "email": "user@example.com",
      "name": "John Doe",
      "posts": [
        {
          "id": "post_456",
          "title": "My First Post"
        }
      ]
    }
  }
}
```
````

## CLI Documentation

````markdown
## Command: deploy

Deploys the application to production.

### Usage

```bash
myapp deploy [options] <environment>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `environment` | Target environment: `staging`, `production` |

### Options

| Option | Alias | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--region` | `-r` | string | `us-east-1` | AWS region |
| `--skip-tests` | - | boolean | `false` | Skip test suite |
| `--verbose` | `-v` | boolean | `false` | Verbose output |
| `--help` | `-h` | boolean | - | Show help |

### Examples

Deploy to production:
```bash
myapp deploy production
```

Deploy to staging in eu-west-1:
```bash
myapp deploy staging --region eu-west-1
```

Deploy with verbose output:
```bash
myapp deploy production --verbose
```

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `AWS_ACCESS_KEY_ID` | Yes | AWS access key |
| `AWS_SECRET_ACCESS_KEY` | Yes | AWS secret key |
| `DEPLOY_WEBHOOK` | No | Webhook URL for notifications |

### Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Deployment failed |
````

## Best Practices

### Include All Status Codes

Document all possible HTTP responses:
- 200 OK
- 201 Created
- 400 Bad Request
- 401 Unauthorized
- 403 Forbidden
- 404 Not Found
- 500 Internal Server Error

### Show Complete Examples

Working, copy-pasteable examples:
```bash
# This should work as-is
curl -X GET "https://api.example.com/users" \
  -H "Authorization: Bearer token" \
  -H "Content-Type: application/json"
```

### Document Error Responses

Show error format and all error codes:
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input",
    "details": [...]
  }
}
```

### Use Tables for Parameters

Organized, scannable format:
| Parameter | Type | Required | Default | Description |

### Include Rate Limits

If applicable:
```markdown
### Rate Limits

- 1000 requests per hour per API key
- 100 requests per minute per IP address
```

### Version Your API

Show which version supports what:
```markdown
## GET /api/v2/users

> Added in v2.0. Replaces [`/api/v1/users`](../v1/users.md)
```

### Link Related Endpoints

Help users discover related functionality:
```markdown
### Related Endpoints

- [`POST /api/users`](#post-users) - Create user
- [`PATCH /api/users/:id`](#patch-users-id) - Update user
- [`DELETE /api/users/:id`](#delete-users-id) - Delete user
```

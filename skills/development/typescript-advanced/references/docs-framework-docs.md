# Framework-Specific Documentation Patterns

## NestJS Documentation

```typescript
/**
 * Authentication guard that validates JWT tokens
 *
 * @guard JwtAuthGuard
 * @usageNotes
 * Apply this guard to controllers or routes that require authentication:
 *
 * @example
 * ```typescript
 * @Controller('users')
 * @UseGuards(JwtAuthGuard)
 * export class UsersController {
 *   @Get()
 *   findAll() {
 *     return this.usersService.findAll();
 *   }
 * }
 * ```
 *
 * @security JWT
 * @throws {UnauthorizedException} When token is invalid or expired
 */
@Injectable()
export class JwtAuthGuard extends AuthGuard('jwt') {}

/**
 * Custom decorator to extract user from request
 *
 * @decorator CurrentUser
 * @usageNotes
 * Use this decorator to access the authenticated user in route handlers
 *
 * @example
 * ```typescript
 * @Get('profile')
 * @UseGuards(JwtAuthGuard)
 * getProfile(@CurrentUser() user: User) {
 *   return user;
 * }
 * ```
 */
export const CurrentUser = createParamDecorator(
  (data: unknown, ctx: ExecutionContext) => {
    const request = ctx.switchToHttp().getRequest();
    return request.user;
  }
);
```

## React Component Documentation

```typescript
/**
 * Reusable button component with various styles and sizes
 *
 * @component Button
 * @param {ButtonProps} props - Component props
 * @param {ReactNode} props.children - Button content
 * @param {'primary' | 'secondary' | 'danger'} props.variant - Visual style
 * @param {'small' | 'medium' | 'large'} props.size - Button size
 * @param {boolean} props.disabled - Whether button is disabled
 * @param {() => void} props.onClick - Click handler
 *
 * @example
 * ```tsx
 * // Primary button
 * <Button variant="primary" onClick={handleSubmit}>
 *   Submit
 * </Button>
 *
 * // Disabled secondary button
 * <Button variant="secondary" disabled>
 *   Cancel
 * </Button>
 * ```
 *
 * @accessibility
 * - Uses semantic <button> element
 * - Disabled state communicated via aria-disabled
 * - Keyboard accessible with Space/Enter keys
 */
export const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'medium',
  disabled = false,
  onClick,
}) => {
  // ...
};

/**
 * Custom hook for managing form state
 *
 * @hook useForm
 * @template T - Form values type
 * @param {T} initialValues - Initial form values
 * @param {(values: T) => void} onSubmit - Submit handler
 * @returns {UseFormReturn<T>} Form state and handlers
 *
 * @example
 * ```tsx
 * const { values, errors, handleChange, handleSubmit } = useForm(
 *   { email: '', password: '' },
 *   (values) => {
 *     api.login(values);
 *   }
 * );
 * ```
 */
export const useForm = <T extends Record<string, unknown>>(
  initialValues: T,
  onSubmit: (values: T) => void
): UseFormReturn<T> => {
  // ...
};
```

## Express Middleware Documentation

```typescript
/**
 * Rate limiting middleware to prevent API abuse
 *
 * @middleware rateLimiter
 * @param {RateLimitOptions} options - Rate limit configuration
 * @param {number} options.windowMs - Time window in milliseconds
 * @param {number} options.maxRequests - Maximum requests per window
 *
 * @errorResponses
 * - 429 Too Many Requests - Rate limit exceeded
 *
 * @example
 * ```typescript
 * app.use('/api', rateLimiter({
 *   windowMs: 15 * 60 * 1000, // 15 minutes
 *   maxRequests: 100
 * }));
 * ```
 *
 * @performance
 * Uses in-memory store by default. For production, use Redis store:
 * ```typescript
 * rateLimiter({
 *   store: new RedisStore({ client: redisClient }),
 *   windowMs: 15 * 60 * 1000,
 *   maxRequests: 100
 * })
 * ```
 */
export const rateLimiter = (options: RateLimitOptions): RequestHandler => {
  // ...
};

/**
 * Error handling middleware
 *
 * @middleware errorHandler
 * @param {Error} err - Error object
 * @param {Request} req - Express request
 * @param {Response} res - Express response
 * @param {NextFunction} next - Next middleware
 *
 * @errorResponses
 * - 400 Bad Request - Validation error
 * - 401 Unauthorized - Authentication error
 * - 404 Not Found - Resource not found
 * - 500 Internal Server Error - Unhandled error
 *
 * @example
 * ```typescript
 * // Must be last middleware
 * app.use(errorHandler);
 * ```
 */
export const errorHandler: ErrorRequestHandler = (err, req, res, next) => {
  // ...
};
```

## Angular Service Documentation

```typescript
/**
 * Service for managing user authentication
 *
 * @injectable UserService
 * @providedIn 'root'
 *
 * @usageNotes
 * Inject this service into components that need authentication:
 *
 * @example
 * ```typescript
 * @Component({ ... })
 * export class LoginComponent {
 *   constructor(private userService: UserService) {}
 *
 *   login() {
 *     this.userService.login(this.credentials).subscribe({
 *       next: (user) => console.log('Logged in', user),
 *       error: (err) => console.error('Login failed', err)
 *     });
 *   }
 * }
 * ```
 */
@Injectable({
  providedIn: 'root',
})
export class UserService {
  /**
   * Authenticates user with credentials
   *
   * @param {Credentials} credentials - User credentials
   * @returns {Observable<User>} Observable that emits authenticated user
   * @throws {HttpErrorResponse} When authentication fails
   */
  login(credentials: Credentials): Observable<User> {
    // ...
  }
}
```

## Vue Composable Documentation

```typescript
/**
 * Composable for fetching and caching data
 *
 * @composable useFetch
 * @template T - Response data type
 * @param {string} url - API endpoint URL
 * @param {FetchOptions} options - Fetch configuration
 * @returns {UseFetchReturn<T>} Reactive fetch state
 *
 * @example
 * ```vue
 * <script setup>
 * const { data, loading, error, refetch } = useFetch('/api/users');
 * </script>
 *
 * <template>
 *   <div v-if="loading">Loading...</div>
 *   <div v-else-if="error">Error: {{ error }}</div>
 *   <ul v-else>
 *     <li v-for="user in data" :key="user.id">{{ user.name }}</li>
 *   </ul>
 * </template>
 * ```
 */
export const useFetch = <T>(
  url: string,
  options?: FetchOptions
): UseFetchReturn<T> => {
  // ...
};
```

## Best Practices

1. **Framework-specific tags** - Use `@component`, `@hook`, `@middleware`, etc.
2. **Usage notes** - Show real-world usage in context
3. **Framework conventions** - Follow framework documentation patterns
4. **Integration examples** - Show how pieces fit together
5. **Lifecycle notes** - Document when hooks/lifecycle methods run
6. **DI/Props documentation** - Clearly document dependencies
7. **State management** - Explain state flow and updates

## Common Patterns

```typescript
/**
 * @component
 * @hook
 * @middleware
 * @guard
 * @decorator
 * @injectable
 * @composable
 * @providedIn
 * @usageNotes
 * @accessibility
 * @performance
 * @errorResponses
 */
```

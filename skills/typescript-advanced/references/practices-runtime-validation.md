# Runtime Validation Best Practices

## Zod Integration

```typescript
import { z } from 'zod';

// Define schema
const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  age: z.number().int().positive().optional(),
  role: z.enum(['admin', 'user', 'guest']),
});

// Infer type from schema
type User = z.infer<typeof UserSchema>;

// Validate at trust boundaries
export const createUser = (data: unknown): User => {
  return UserSchema.parse(data); // Throws on validation failure
};

// Safe parsing for user input
export const validateUser = (data: unknown): Result<User, z.ZodError> => {
  const result = UserSchema.safeParse(data);
  if (result.success) {
    return { ok: true, value: result.data };
  }
  return { ok: false, error: result.error };
};
```

## Schema Composition

```typescript
// Base schemas
const AddressSchema = z.object({
  street: z.string(),
  city: z.string(),
  zipCode: z.string().regex(/^\d{5}$/),
});

const ContactSchema = z.object({
  email: z.string().email(),
  phone: z.string().optional(),
});

// Compose schemas
const CustomerSchema = z.object({
  name: z.string(),
  address: AddressSchema,
  contact: ContactSchema,
  metadata: z.record(z.string(), z.unknown()),
});

// Extend schemas
const PremiumCustomerSchema = CustomerSchema.extend({
  tier: z.literal('premium'),
  discount: z.number().min(0).max(100),
});

// Pick/omit fields
const PublicCustomerSchema = CustomerSchema.pick({
  name: true,
  contact: true,
});
```

## Transform and Refine

```typescript
// Transform data during validation
const DateSchema = z.string().transform((str) => new Date(str));

const NormalizedEmailSchema = z
  .string()
  .email()
  .transform((email) => email.toLowerCase().trim());

// Custom validation with refine
const PasswordSchema = z
  .string()
  .min(8)
  .refine((pwd) => /[A-Z]/.test(pwd), {
    message: 'Must contain uppercase letter',
  })
  .refine((pwd) => /[0-9]/.test(pwd), {
    message: 'Must contain number',
  });

// Complex validation
const RangeSchema = z
  .object({
    min: z.number(),
    max: z.number(),
  })
  .refine((data) => data.max > data.min, {
    message: 'Max must be greater than min',
    path: ['max'],
  });
```

## API Request Validation

```typescript
// Express middleware
import { Request, Response, NextFunction } from 'express';

const validateRequest = <T>(schema: z.Schema<T>) => {
  return (req: Request, res: Response, next: NextFunction) => {
    const result = schema.safeParse(req.body);
    if (!result.success) {
      return res.status(400).json({
        error: 'Validation failed',
        details: result.error.format(),
      });
    }
    req.body = result.data;
    next();
  };
};

// Usage
app.post('/users', validateRequest(UserSchema), (req, res) => {
  const user = req.body; // Type-safe and validated
  // ... create user
});
```

## Configuration Validation

```typescript
// Environment configuration
const ConfigSchema = z.object({
  NODE_ENV: z.enum(['development', 'production', 'test']),
  PORT: z.string().transform((s) => parseInt(s, 10)),
  DATABASE_URL: z.string().url(),
  LOG_LEVEL: z.enum(['debug', 'info', 'warn', 'error']).default('info'),
  FEATURE_FLAGS: z
    .string()
    .transform((s) => s.split(','))
    .default(''),
});

// Load and validate at startup
export const config = ConfigSchema.parse(process.env);
```

## Best Practices

1. **Validate at trust boundaries** - Parse external data (API requests, file uploads, env vars)
2. **Use schemas as source of truth** - Derive types with `z.infer<>`
3. **Prefer safeParse for user input** - Return results instead of throwing
4. **Compose schemas** - Build complex schemas from simple ones
5. **Add custom error messages** - Make validation errors actionable
6. **Transform during validation** - Normalize data (lowercase emails, parse dates)
7. **Validate early** - Fail fast at application boundaries
8. **Test validation logic** - Write tests for schema validation

## Common Patterns

```typescript
// Optional with default
const SettingsSchema = z.object({
  theme: z.enum(['light', 'dark']).default('light'),
  language: z.string().default('en'),
});

// Nullable vs optional
const ProfileSchema = z.object({
  bio: z.string().optional(), // Field may be missing
  avatar: z.string().nullable(), // Field present but may be null
  website: z.string().nullish(), // May be missing or null
});

// Discriminated union validation
const EventSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('click'), x: z.number(), y: z.number() }),
  z.object({ type: z.literal('keypress'), key: z.string() }),
]);

// Recursive schemas
type Category = {
  name: string;
  subcategories: Category[];
};

const CategorySchema: z.ZodType<Category> = z.lazy(() =>
  z.object({
    name: z.string(),
    subcategories: z.array(CategorySchema),
  })
);
```

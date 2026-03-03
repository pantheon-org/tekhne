# Type-Safe Form Validation

Building type-safe form validation with field-level and form-level validation.

## Basic Form Validation

```typescript
type ValidationError = string;
type ValidationResult<T> = 
  | { success: true; value: T }
  | { success: false; errors: Record<keyof T, ValidationError> };

type Validator<T> = (value: T) => ValidationError | null;

interface FormValidators<T> {
  [K in keyof T]: Validator<T[K]>;
}

function createForm<T extends Record<string, unknown>>(
  validators: FormValidators<T>
) {
  return {
    validate(data: T): ValidationResult<T> {
      const errors: Partial<Record<keyof T, ValidationError>> = {};
      let hasErrors = false;

      for (const key in validators) {
        const error = validators[key](data[key]);
        if (error) {
          errors[key] = error;
          hasErrors = true;
        }
      }

      if (hasErrors) {
        return {
          success: false,
          errors: errors as Record<keyof T, ValidationError>
        };
      }

      return { success: true, value: data };
    }
  };
}

// Usage
interface UserForm {
  name: string;
  email: string;
  age: number;
}

const userForm = createForm<UserForm>({
  name: (value) => value.length < 2 ? 'Name too short' : null,
  email: (value) => !value.includes('@') ? 'Invalid email' : null,
  age: (value) => value < 18 ? 'Must be 18+' : null
});

const result = userForm.validate({
  name: 'John',
  email: 'john@example.com',
  age: 25
});

if (result.success) {
  console.log('Valid:', result.value);
} else {
  console.log('Errors:', result.errors);
}
```

## With Async Validation

```typescript
type AsyncValidator<T> = (value: T) => Promise<ValidationError | null>;

interface AsyncFormValidators<T> {
  [K in keyof T]: Validator<T[K]> | AsyncValidator<T[K]>;
}

function createAsyncForm<T extends Record<string, unknown>>(
  validators: AsyncFormValidators<T>
) {
  return {
    async validate(data: T): Promise<ValidationResult<T>> {
      const errors: Partial<Record<keyof T, ValidationError>> = {};
      let hasErrors = false;

      for (const key in validators) {
        const error = await validators[key](data[key]);
        if (error) {
          errors[key] = error;
          hasErrors = true;
        }
      }

      if (hasErrors) {
        return {
          success: false,
          errors: errors as Record<keyof T, ValidationError>
        };
      }

      return { success: true, value: data };
    }
  };
}

// Usage with async validators
const asyncUserForm = createAsyncForm<UserForm>({
  name: (value) => value.length < 2 ? 'Name too short' : null,
  email: async (value) => {
    const exists = await checkEmailExists(value);
    return exists ? 'Email already taken' : null;
  },
  age: (value) => value < 18 ? 'Must be 18+' : null
});
```

## With Field Dependencies

```typescript
type CrossFieldValidator<T> = (data: T) => Record<keyof T, ValidationError> | null;

function createFormWithCrossValidation<T extends Record<string, unknown>>(
  validators: FormValidators<T>,
  crossValidators: CrossFieldValidator<T>[]
) {
  return {
    validate(data: T): ValidationResult<T> {
      const errors: Partial<Record<keyof T, ValidationError>> = {};
      let hasErrors = false;

      // Field-level validation
      for (const key in validators) {
        const error = validators[key](data[key]);
        if (error) {
          errors[key] = error;
          hasErrors = true;
        }
      }

      // Cross-field validation
      for (const crossValidator of crossValidators) {
        const crossErrors = crossValidator(data);
        if (crossErrors) {
          Object.assign(errors, crossErrors);
          hasErrors = true;
        }
      }

      if (hasErrors) {
        return {
          success: false,
          errors: errors as Record<keyof T, ValidationError>
        };
      }

      return { success: true, value: data };
    }
  };
}

// Usage
interface PasswordForm {
  password: string;
  confirmPassword: string;
}

const passwordForm = createFormWithCrossValidation<PasswordForm>(
  {
    password: (value) => value.length < 8 ? 'Password too short' : null,
    confirmPassword: () => null // Validated in cross-field
  },
  [
    (data) => {
      if (data.password !== data.confirmPassword) {
        return { confirmPassword: 'Passwords do not match' };
      }
      return null;
    }
  ]
);
```

## With Zod Integration

```typescript
import { z } from 'zod';

function createZodForm<T extends z.ZodRawShape>(schema: z.ZodObject<T>) {
  type FormData = z.infer<z.ZodObject<T>>;
  
  return {
    validate(data: FormData): ValidationResult<FormData> {
      const result = schema.safeParse(data);
      
      if (result.success) {
        return { success: true, value: result.data };
      }
      
      const errors: Partial<Record<keyof FormData, ValidationError>> = {};
      result.error.errors.forEach(err => {
        if (err.path.length > 0) {
          errors[err.path[0] as keyof FormData] = err.message;
        }
      });
      
      return {
        success: false,
        errors: errors as Record<keyof FormData, ValidationError>
      };
    }
  };
}

// Usage
const userSchema = z.object({
  name: z.string().min(2, 'Name too short'),
  email: z.string().email('Invalid email'),
  age: z.number().min(18, 'Must be 18+')
});

const zodForm = createZodForm(userSchema);
const result = zodForm.validate({
  name: 'John',
  email: 'john@example.com',
  age: 25
});
```

## Best Practices

1. **Type Safety**: Use generics to ensure validators match form fields
2. **Async Validation**: Support async validators for server-side checks
3. **Cross-Field Validation**: Validate field dependencies explicitly
4. **Error Messages**: Provide clear, actionable error messages
5. **Integration**: Use validation libraries like Zod for complex schemas

## See Also

- practices-runtime-validation.md
- guards-type-predicates.md
- patterns-state-machine.md

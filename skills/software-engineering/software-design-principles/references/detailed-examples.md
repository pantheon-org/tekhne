# Detailed Implementation Examples

This file contains comprehensive code examples for applying software design principles across different programming paradigms.

## Table of Contents
- [Dependency Direction Examples](#dependency-direction-examples)
- [Entity Design Examples](#entity-design-examples)
- [SOLID Principles Examples](#solid-principles-examples)
- [Structural Design Examples](#structural-design-examples)
- [Real-World Violation Examples](#real-world-violation-examples)

## Dependency Direction Examples

### TypeScript Clean Architecture
```typescript
// GOOD: Domain entity with no external dependencies
export interface User {
  id: UserId;
  email: Email;
  validateEmailChange(newEmail: Email): ValidationResult;
}

// GOOD: Application service depends on domain
export class UserService {
  constructor(
    private userRepository: UserRepository, // Interface from domain
    private emailService: EmailService     // Interface from domain
  ) {}
}

// BAD: Domain entity importing infrastructure
import { Database } from '@/infrastructure/database'; // ❌ Wrong direction
export class User {
  save() {
    return Database.insert(this); // ❌ Domain knows about infrastructure
  }
}
```

### Elixir Boundary Management
```elixir
# GOOD: Domain context with pure business logic
defmodule Billing.Invoice do
  @type t :: %__MODULE__{
    id: String.t(),
    amount: Money.t(),
    status: :draft | :sent | :paid
  }
  
  def calculate_total(%__MODULE__{} = invoice) do
    # Pure business calculation
    invoice.line_items
    |> Enum.reduce(Money.new(0), &Money.add(&2, &1.amount))
  end
end

# GOOD: Infrastructure adapter
defmodule Billing.Adapters.InvoiceRepository do
  def save(%Invoice{} = invoice) do
    # Database operations here
  end
end

# BAD: Domain importing infrastructure
defmodule Billing.Invoice do
  alias Billing.Database.Repo # ❌ Wrong direction
  
  def save(%__MODULE__{} = invoice) do
    Repo.insert(invoice) # ❌ Domain knows about persistence
  end
end
```

## Entity Design Examples

### Rich Domain Entities
```typescript
// GOOD: Rich entity with business logic
export class Order {
  private constructor(
    private readonly id: OrderId,
    private readonly customerId: CustomerId,
    private items: OrderItem[],
    private status: OrderStatus
  ) {}
  
  static create(customerId: CustomerId, items: OrderItem[]): Result<Order, DomainError> {
    if (items.length === 0) {
      return Err(new DomainError("Order must have at least one item"));
    }
    
    return Ok(new Order(
      OrderId.generate(),
      customerId, 
      items,
      OrderStatus.PENDING
    ));
  }
  
  addItem(item: OrderItem): Result<void, DomainError> {
    if (this.status !== OrderStatus.PENDING) {
      return Err(new DomainError("Cannot modify confirmed order"));
    }
    
    this.items.push(item);
    return Ok(void 0);
  }
  
  calculateTotal(): Money {
    return this.items.reduce(
      (total, item) => total.add(item.getPrice()),
      Money.zero()
    );
  }
}

// BAD: Anemic entity
export class Order {
  public id: string;
  public customerId: string;
  public items: any[];
  public status: string;
  public total: number; // ❌ No business logic, just data
}
```

### Elixir Rich Entities with Changesets
```elixir
# GOOD: Rich entity with validation
defmodule Accounts.User do
  use Ecto.Schema
  import Ecto.Changeset
  
  @type t :: %__MODULE__{
    email: String.t(),
    encrypted_password: String.t(),
    status: :active | :suspended | :deleted
  }
  
  def registration_changeset(attrs \\ %{}) do
    %__MODULE__{}
    |> cast(attrs, [:email, :password])
    |> validate_required([:email, :password])
    |> validate_email_format()
    |> validate_password_strength()
    |> encrypt_password()
  end
  
  def suspend(%__MODULE__{status: :active} = user) do
    user
    |> change(status: :suspended)
    |> validate_suspension_allowed()
  end
  
  def suspend(%__MODULE__{status: status}) do
    {:error, "Cannot suspend user with status: #{status}"}
  end
  
  # Business rule enforcement
  defp validate_suspension_allowed(changeset) do
    # Check business rules for suspension
    case get_field(changeset, :subscription_status) do
      :premium -> add_error(changeset, :status, "Premium users cannot be suspended")
      _ -> changeset
    end
  end
end
```

## SOLID Principles Examples

### Single Responsibility Principle
```typescript
// BAD: Violates SRP
export class UserManager {
  validateUser(user: User): boolean { /* validation */ }
  saveUser(user: User): void { /* database operation */ }
  sendWelcomeEmail(user: User): void { /* email sending */ }
  hashPassword(password: string): string { /* cryptography */ }
  logUserActivity(user: User, action: string): void { /* logging */ }
}

// GOOD: Separated responsibilities
export class UserValidator {
  validate(user: User): ValidationResult {
    // Only validation logic
  }
}

export class UserRepository {
  save(user: User): Promise<void> {
    // Only persistence logic
  }
}

export class UserNotificationService {
  sendWelcomeEmail(user: User): Promise<void> {
    // Only notification logic
  }
}
```

### Open/Closed Principle
```typescript
// BAD: Must modify for new discount types
class DiscountCalculator {
  calculate(type: string, amount: number): number {
    switch (type) {
      case 'percentage':
        return amount * 0.1;
      case 'fixed':
        return Math.min(amount, 50);
      // ❌ Adding new types requires modification
      default:
        return 0;
    }
  }
}

// GOOD: Open for extension, closed for modification
interface DiscountStrategy {
  calculate(amount: number): number;
}

class PercentageDiscount implements DiscountStrategy {
  constructor(private rate: number) {}
  
  calculate(amount: number): number {
    return amount * this.rate;
  }
}

class FixedAmountDiscount implements DiscountStrategy {
  constructor(private discountAmount: number) {}
  
  calculate(amount: number): number {
    return Math.min(amount, this.discountAmount);
  }
}

class DiscountCalculator {
  calculate(strategy: DiscountStrategy, amount: number): number {
    return strategy.calculate(amount);
  }
}
```

## Real-World Violation Examples

### GraphQL Resolver Violations
```typescript
// BAD: Violates multiple principles
export class UserResolver {
  async getUser(id: string) {
    // ❌ SRP violation: validation, business logic, data access all mixed
    
    // Manual validation (should be in validator)
    if (!id || id.length < 1) {
      throw new Error("Invalid ID");
    }
    
    // Database access (should be in repository)
    const user = await database.users.findOne({ id });
    if (!user) {
      throw new Error("User not found");
    }
    
    // Business logic (should be in entity/service)
    if (user.subscription_expires < new Date()) {
      user.status = 'expired';
      await database.users.update({ id }, { status: 'expired' });
    }
    
    // Email sending (should be in notification service)
    if (user.last_login < thirtyDaysAgo()) {
      await emailService.send(user.email, 'comeback-email');
    }
    
    // Logging (should be in logging service)  
    await database.audit_log.insert({
      user_id: id,
      action: 'user_accessed',
      timestamp: new Date()
    });
    
    // Data transformation (should be in presenter/mapper)
    return {
      id: user.id,
      name: user.first_name + ' ' + user.last_name,
      email: user.email.toLowerCase(),
      avatar: `https://cdn.example.com/avatars/${user.id}.jpg`
    };
  }
}

// GOOD: Properly separated
export class UserResolver {
  constructor(
    private userService: UserService,
    private userPresenter: UserPresenter
  ) {}
  
  async getUser(id: string): Promise<UserDto> {
    const user = await this.userService.getById(new UserId(id));
    return this.userPresenter.toDto(user);
  }
}
```

### React Component Violations
```tsx
// BAD: Component doing too much
const UserDashboard: React.FC<{ userId: string }> = ({ userId }) => {
  const [user, setUser] = useState<User>();
  const [subscription, setSubscription] = useState<Subscription>();
  const [notifications, setNotifications] = useState<Notification[]>([]);
  
  useEffect(() => {
    // ❌ Data fetching logic in component
    fetch(`/api/users/${userId}`)
      .then(response => response.json())
      .then(userData => {
        setUser(userData);
        
        // ❌ Business logic in component
        if (userData.subscriptionId) {
          fetch(`/api/subscriptions/${userData.subscriptionId}`)
            .then(subResponse => subResponse.json())
            .then(subData => {
              setSubscription(subData);
              
              // ❌ More business logic
              if (subData.expires < Date.now()) {
                // Send expiry email
                fetch('/api/notifications/expiry', {
                  method: 'POST',
                  body: JSON.stringify({ userId, email: userData.email })
                });
              }
            });
        }
        
        // ❌ Analytics tracking in component
        analytics.track('user_dashboard_viewed', {
          userId,
          timestamp: Date.now()
        });
      });
  }, [userId]);
  
  // ❌ Complex rendering logic mixed with business logic
  return (
    <div>
      {user && (
        <>
          <h1>Welcome {user.firstName} {user.lastName}</h1>
          {subscription?.expires < Date.now() ? (
            <div className="alert-danger">
              Your subscription expired! 
              <button onClick={() => {
                // ❌ Payment logic in component
                stripe.redirectToCheckout({
                  sessionId: subscription.renewalSessionId
                });
              }}>
                Renew Now
              </button>
            </div>
          ) : (
            <div className="subscription-active">
              Active until {new Date(subscription.expires).toLocaleDateString()}
            </div>
          )}
        </>
      )}
    </div>
  );
};

// GOOD: Separated concerns
const UserDashboard: React.FC<{ userId: string }> = ({ userId }) => {
  const { user, subscription, loading } = useUserDashboardData(userId);
  
  if (loading) return <LoadingSpinner />;
  
  return (
    <div>
      <UserHeader user={user} />
      <SubscriptionStatus subscription={subscription} />
    </div>
  );
};
```

## Structural Design Examples

### Composition Over Inheritance
```typescript
// BAD: Inheritance hierarchy
class Animal {
  move() { /* basic movement */ }
  eat() { /* basic eating */ }
}

class Bird extends Animal {
  fly() { /* flying logic */ }
  move() { /* override with flying */ }
}

class Penguin extends Bird {
  fly() { throw new Error("Penguins can't fly!"); } // ❌ LSP violation
  move() { /* swimming logic */ }
}

// GOOD: Composition
interface MovementBehavior {
  move(): void;
}

class FlyingMovement implements MovementBehavior {
  move(): void { /* flying logic */ }
}

class SwimmingMovement implements MovementBehavior {
  move(): void { /* swimming logic */ }
}

class Animal {
  constructor(private movementBehavior: MovementBehavior) {}
  
  move(): void {
    this.movementBehavior.move();
  }
}

// Usage
const eagle = new Animal(new FlyingMovement());
const penguin = new Animal(new SwimmingMovement());
```

### Law of Demeter Examples
```typescript
// BAD: Violates Law of Demeter
class Customer {
  getOrder() {
    return this.order;
  }
}

class Order {
  getDelivery() {
    return this.delivery;
  }
}

class Delivery {
  getAddress() {
    return this.address;
  }
}

// ❌ Client reaches through multiple objects
const customerCity = customer.getOrder().getDelivery().getAddress().city;

// GOOD: Delegate to owning objects
class Customer {
  getDeliveryCity(): string {
    return this.order.getDeliveryCity();
  }
}

class Order {
  getDeliveryCity(): string {
    return this.delivery.getCity();
  }
}

class Delivery {
  getCity(): string {
    return this.address.city;
  }
}

// ✅ Clean interface
const customerCity = customer.getDeliveryCity();
```

This reference file provides comprehensive examples that can be referenced from the main SKILL.md file without cluttering it with detailed code samples.
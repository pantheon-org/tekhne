# Scenario 02: Shopping Cart Bug Fix

## User Prompt

EcommerceFlow's shopping cart system has been causing customer complaints due to incorrect total calculations. The QA team has identified specific scenarios where the cart total is wrong: bulk discounts aren't being applied correctly, and tax calculations seem inconsistent across different product categories.

The customer support team has escalated this as a high-priority issue because customers are abandoning purchases at checkout when they see unexpected totals. The business team suspects the issue is in the cart calculation logic, but they need the bug reproduced and fixed quickly to prevent further revenue loss.

Your task is to:

1. Investigate and reproduce the bug with failing tests first
2. Fix the cart calculation logic
3. Ensure your solution handles various edge cases

The system should calculate cart totals considering:
- Item prices and quantities
- Bulk discounts (10% off orders over $100)
- Tax rates (8% on most items, 0% on books)
- Shipping costs ($5 flat rate, free over $50)

Required files:
- Cart calculation implementation
- Comprehensive test suite that reproduces the bug
- Documentation of the bug and fix approach

## Input Files

The following existing cart implementation is provided. Extract it before beginning.

```typescript
// inputs/cart.ts
export class ShoppingCart {
  private items: CartItem[] = [];

  addItem(item: CartItem): void {
    this.items.push(item);
  }

  calculateTotal(): number {
    const subtotal = this.items.reduce((sum, item) =>
      sum + (item.price * item.quantity), 0);

    const discount = subtotal > 100 ? subtotal * 0.1 : 0;
    const tax = this.items.reduce((sum, item) =>
      sum + (item.category === 'books' ? 0 : item.price * item.quantity * 0.08), 0);
    const shipping = subtotal > 50 ? 0 : 5;

    // Bug: discount not being subtracted correctly
    return subtotal + tax + shipping;
  }

  getItems(): CartItem[] {
    return this.items;
  }
}

export interface CartItem {
  name: string;
  price: number;
  quantity: number;
  category: string;
}
```

## Expected Behavior

1. Create a failing test that demonstrates the specific bug before fixing it
2. Focus tests on observable cart calculation behavior — not internal implementation details
3. Name tests clearly to describe the scenario and expected behavior (e.g., `should_X_when_Y`)
4. Verify one logical concept per test — not multiple different calculations
5. Use specific matcher methods rather than generic equality checks (e.g., `toBeCloseTo` for floating point numbers)
6. Cover boundary conditions like exactly $50, exactly $100, and zero quantities
7. Follow logical Arrange-Act-Assert organization in each test
8. Make each test independent and not reliant on state from other tests
9. Show that previously failing tests now pass after the fix

## Success Criteria

- **Bug reproduction test**: Creates a failing test that demonstrates the specific bug before fixing it
- **Behavior-focused tests**: Tests focus on observable cart calculation behavior, not internal implementation details
- **Descriptive test naming**: Test names clearly describe scenario and expected behavior using patterns like `should_X_when_Y`
- **One assertion per test**: Each test verifies one logical concept, not multiple different calculations
- **Specific assertions used**: Uses specific matcher methods rather than generic equality checks (e.g., `toBeCloseTo` for numbers)
- **Edge case coverage**: Tests cover boundary conditions like exactly $50, exactly $100, zero quantities
- **Clear test structure**: Tests follow logical Arrange-Act-Assert organization
- **Isolated test cases**: Each test is independent and doesn't rely on state from other tests
- **Bug fix verification**: Shows that previously failing tests now pass after the fix

## Failure Conditions

- No failing test reproduces the bug before the fix is applied
- Tests check internal implementation details rather than observable calculation outcomes
- Test names are generic and do not describe the scenario or expected outcome
- A single test verifies multiple unrelated calculation rules simultaneously
- Generic equality checks are used instead of appropriate matchers for numeric comparisons
- Boundary conditions ($50, $100 thresholds) are not tested
- Tests do not follow Arrange-Act-Assert structure
- Tests share state or depend on execution order
- No demonstration that previously failing tests pass after the fix

# Shopping Cart Bug Fix

## Problem/Feature Description

EcommerceFlow's shopping cart system has been causing customer complaints due to incorrect total calculations. The QA team has identified specific scenarios where the cart total is wrong: bulk discounts aren't being applied correctly, and tax calculations seem inconsistent across different product categories.

The customer support team has escalated this as a high-priority issue because customers are abandoning purchases at checkout when they see unexpected totals. The business team suspects the issue is in the cart calculation logic, but they need the bug reproduced and fixed quickly to prevent further revenue loss.

## Output Specification

Your task is to:

1. Investigate and reproduce the bug with failing tests first
2. Fix the cart calculation logic
3. Ensure your solution handles various edge cases

The system should calculate cart totals considering:

- Item prices and quantities  
- Bulk discounts (10% off orders over $100)
- Tax rates (8% on most items, 0% on books)
- Shipping costs ($5 flat rate, free over $50)

**Required files:**

- Cart calculation implementation  
- Comprehensive test suite that reproduces the bug
- Documentation of the bug and fix approach

## Input Files

Extract the following existing cart implementation before beginning:

**FILE**: inputs/cart.ts

```typescript
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

# Scenario 2: Shopping Cart Bug Fix

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

# Multi-Step Order Processing System

## Problem/Feature Description

RetailPlus has outgrown their simple order system and needs a sophisticated multi-step order processing workflow. Orders now require inventory validation, payment processing, fraud detection, shipping calculation, and warehouse notification. Each step can fail independently and requires different retry and rollback strategies.

The business team has defined a complex workflow: orders start in "pending" status, move to "validated" after inventory and fraud checks, then "paid" after payment processing, then "fulfilled" after shipping arrangements, and finally "completed" when shipped. Failed steps should move orders to appropriate error states with clear failure reasons.

The existing system has grown organically with several issues: tests that combine multiple workflow steps making failures hard to diagnose, shared test data that causes tests to affect each other, and complex test scenarios that are difficult to understand and maintain.

## Output Specification

Implement a comprehensive order processing system with:

1. Multi-step workflow management (pending → validated → paid → fulfilled → completed)
2. Individual step validation (inventory, fraud, payment, shipping)
3. Error handling and rollback for each step
4. Order status tracking and history
5. Integration points for external services (payment gateway, shipping API)

System should handle:

- Concurrent order processing
- Partial failures and recovery
- Business rule validation at each step
- Audit trail of all status changes

**Required files:**

- Order processing workflow implementation  
- Individual step processors
- Status management system
- Comprehensive test suite demonstrating each workflow step
- Integration examples showing the complete flow

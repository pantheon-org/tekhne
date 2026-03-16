# Task: Apply Anti-Corruption Layer to Isolate an External System

Your domain has a clean `Customer` model but the external CRM API returns poorly named, flat data. Apply the Anti-Corruption Layer pattern to prevent the external model from polluting your domain.

## Domain Model

```typescript
// src/domain/Customer.ts
export interface Customer {
  id: string
  fullName: string
  email: string
  isPremium: boolean
  createdAt: Date
}
```

## External CRM API Response Shape

```typescript
// external/crm-api response (cannot be modified)
interface CrmApiResponse {
  cust_id: number          // numeric, not a UUID
  f_name: string
  l_name: string           // first and last separate
  email_addr: string
  acct_type: 'STD' | 'PRE' // 'PRE' means premium
  reg_ts: string           // ISO timestamp string
}
```

## Output Specification

Produce:

1. `CrmGateway.ts` — an Anti-Corruption Layer class with a method `getCustomer(id: string): Promise<Customer>`. The implementation may stub the actual HTTP call (just accept raw `CrmApiResponse` data as a parameter for testing purposes, or use a hardcoded mock response). All translation logic must be inside this class.
2. `acl-analysis.md` — explain what domain pollution would look like without the ACL (one paragraph), and how the ACL prevents it (one paragraph).

Rules:
- Domain code outside `CrmGateway.ts` must never reference `CrmApiResponse` types.
- The `Customer` domain type must be constructed inside `CrmGateway.ts`, not outside.

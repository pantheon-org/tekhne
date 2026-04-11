# Scenario 03: Apply Anti-Corruption Layer to Isolate an External System

## User Prompt

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

## Expected Behavior

1. The `getCustomer` method (or equivalent) returns a `Customer` object with all five fields: `id`, `fullName`, `email`, `isPremium`, `createdAt`
2. `CrmGateway.ts` concatenates `f_name` and `l_name` into `fullName`
3. `CrmGateway.ts` maps `'PRE'` to `true` and `'STD'` to `false` for the `isPremium` field
4. `CrmGateway.ts` converts the `reg_ts` string to a `Date` object for the `createdAt` field
5. `CrmGateway.ts` does not export `CrmApiResponse` or return it; only `Customer` is returned to callers
6. `acl-analysis.md` describes what pollution would look like (e.g. `cust_id` and `acct_type` used in domain code) and how the ACL isolates it

## Success Criteria

- **CrmGateway.ts produces a Customer domain object**: The `getCustomer` method (or equivalent) returns a `Customer` object with all five fields: `id`, `fullName`, `email`, `isPremium`, `createdAt`
- **Full name is assembled from f_name and l_name**: `CrmGateway.ts` concatenates `f_name` and `l_name` into `fullName` — not just using one field
- **acct_type is correctly mapped to isPremium**: `CrmGateway.ts` maps `'PRE'` to `true` and `'STD'` to `false` for the `isPremium` field
- **reg_ts is converted to a Date**: `CrmGateway.ts` converts the `reg_ts` string to a `Date` object for the `createdAt` field (e.g. `new Date(reg_ts)`)
- **No CrmApiResponse fields leak outside CrmGateway.ts**: `CrmGateway.ts` does not export `CrmApiResponse` or return it; only `Customer` is returned to callers
- **acl-analysis.md explains domain pollution risk and ACL resolution**: `acl-analysis.md` contains one paragraph on what pollution would look like (e.g. `cust_id` and `acct_type` used in domain code) and one paragraph on how the ACL isolates it

## Failure Conditions

- `getCustomer` does not return all five `Customer` fields, or returns the raw CRM data structure
- `fullName` is set to only `f_name` or only `l_name` instead of the concatenated full name
- `acct_type` is not mapped: `isPremium` is `true` for both `'STD'` and `'PRE'`, or the field is absent
- `reg_ts` is not converted to a `Date` object, leaving it as a string in the domain model
- `CrmApiResponse` fields or types are exported or returned to callers outside `CrmGateway.ts`
- `acl-analysis.md` is missing or does not explain the domain pollution risk and ACL resolution

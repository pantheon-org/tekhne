# Scenario 3: Apply Anti-Corruption Layer to Isolate an External System

## User Prompt

Your domain has a clean `Customer` model but the external CRM API returns poorly named, flat data. Apply the Anti-Corruption Layer pattern to prevent the external model from polluting your domain.

## Expected Behavior

1. The `getCustomer` method (or equivalent) returns a `Customer` object with all five fields: `id`, `fullName`, `email`, `isPremium`, `createdAt`
2. `CrmGateway.ts` concatenates `f_name` and `l_name` into `fullName`
3. `CrmGateway.ts` maps `'PRE'` to `true` and `'STD'` to `false` for the `isPremium` field
4. `CrmGateway.ts` converts the `reg_ts` string to a `Date` object for the `createdAt` field
5. `CrmGateway.ts` does not export `CrmApiResponse` or return it; only `Customer` is returned to callers
6. `acl-analysis.md` describes what pollution would look like (e.g. `cust_id` and `acct_type` used in domain code) and how the ACL isolates it

## Failure Conditions

- `getCustomer` does not return all five `Customer` fields, or returns the raw CRM data structure
- `fullName` is set to only `f_name` or only `l_name` instead of the concatenated full name
- `acct_type` is not mapped: `isPremium` is `true` for both `'STD'` and `'PRE'`, or the field is absent
- `reg_ts` is not converted to a `Date` object, leaving it as a string in the domain model
- `CrmApiResponse` fields or types are exported or returned to callers outside `CrmGateway.ts`
- `acl-analysis.md` is missing or does not explain the domain pollution risk and ACL resolution

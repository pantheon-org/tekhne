# AWS Console Reference

## SSO Authentication Flow

### Step 1 — Open the SSO portal

Navigate to your organisation's AWS IAM Identity Center portal URL.
Wait for the `AWS IAM Identity Center` heading or account list to appear.

### Step 2 — Select the account

The portal lists accounts. Use the search box or scroll to find the target account name or ID.
Click the account row to expand it and reveal available permission sets (roles).

### Step 3 — Open the console for the correct role

Click the role name (e.g. `PowerUserAccess`, `AdministratorAccess`) under the account.
A new browser tab opens with the AWS Management Console for that account + role.

**Banner dismissal — do this before navigating further:**

- Cookie banner: click `Accept all` or `Reject all`
- New-experience banner: click `X` or `Dismiss`

### Step 4 — Set the region

The region selector is in the top-right corner of the console header.
Click it, type or select the target region (e.g. `EU (Ireland)` → `eu-west-1`).
Confirm `region=eu-west-1` appears in the URL after switching.

---

## Region Codes Reference

| Console display name  | Region code       |
|-----------------------|-------------------|
| EU (Ireland)          | eu-west-1         |
| EU (Frankfurt)        | eu-central-1      |
| US East (N. Virginia) | us-east-1         |
| US West (Oregon)      | us-west-2         |
| AP (Sydney)           | ap-southeast-2    |
| AP (Tokyo)            | ap-northeast-1    |
| AP (Singapore)        | ap-southeast-1    |

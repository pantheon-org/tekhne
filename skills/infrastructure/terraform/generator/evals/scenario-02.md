# Scenario 02: Multi-Region Notification Service with Remote State

## User Prompt

A product team needs Terraform to manage identical infrastructure for three regional deployments of a `notification-service`: `us-east-1`, `eu-west-1`, and `ap-southeast-1`. Each region needs:

- An SQS queue (standard, not FIFO) for inbound notifications, named after the region
- An SNS topic that fans out to that queue

The team works in squads of 6 engineers, uses GitHub Actions for CI/CD, and all environments are managed from a shared Git repository. Currently there is no infrastructure state management beyond local files, which has caused several incidents where two engineers applied conflicting changes simultaneously. The lead engineer wants this fixed as part of this task.

The infrastructure should be easy to extend — a fourth region may be added in the future without restructuring the code.

Generate the Terraform configuration for this setup. The solution should handle all three regions' resources within a single configuration. Include a backend configuration appropriate for a team workflow. After generating files, include usage instructions.

## Expected Behavior

1. Use `for_each = toset(...)` or `for_each` over a map for `aws_sqs_queue` resources — NOT `count = length(...)`
2. Use `for_each` for `aws_sns_topic` resources as well — NOT `count`
3. Include a `backend` block (e.g., `backend "s3" {}`) — the configuration does NOT rely on local state
4. Include a `dynamodb_table` argument in the S3 backend (or equivalent remote backend) for state locking
5. NOT use `count.index` combined with a list variable to generate resource names
6. Include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf` in generated files
7. Adding a fourth region requires only adding to a variable/map — not adding a new resource block
8. Response includes next steps covering at least `terraform init`, `plan`, and `apply`

## Success Criteria

- **for_each used for SQS queues**: The `aws_sqs_queue` resources use `for_each = toset(...)` or `for_each` over a map — NOT `count = length(...)`
- **for_each used for SNS topics**: The `aws_sns_topic` resources also use `for_each` — NOT `count`
- **Remote backend configured**: A `backend` block is present (e.g., `backend "s3" {}`) — the configuration does NOT rely on local state
- **State locking configured**: The S3 backend (or equivalent remote backend) includes a `dynamodb_table` argument for state locking
- **No count with index access**: The configuration does NOT use `count.index` combined with a list variable to generate resource names
- **File organization correct**: Generated files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
- **Extensibility supported**: Adding a fourth region requires only adding to a variable/map — not adding a new resource block
- **Usage instructions included**: Response includes next steps covering at least `terraform init`, `plan`, and `apply`

## Failure Conditions

- `count` is used instead of `for_each` for SQS queues or SNS topics
- No remote backend is configured (local state used)
- State locking via DynamoDB is absent from the backend configuration
- `count.index` is used to generate resource names from a list variable
- Required files (`main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`) are absent
- Adding a fourth region would require adding a new resource block rather than a variable update

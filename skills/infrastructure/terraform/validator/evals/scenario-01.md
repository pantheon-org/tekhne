# Scenario 01: Validate a Terraform Web Application Module

## User Prompt

A platform engineering team has written a new Terraform configuration for provisioning a web application stack on AWS. The configuration defines an EC2 instance, a security group, an S3 bucket for static assets, and an RDS instance. Before opening a pull request and running this against a real AWS account, the team lead has asked you to perform a thorough offline validation — catching any formatting problems, syntax issues, misconfigurations, and security concerns without actually deploying anything.

You should produce a written validation report covering every check you performed and its result. The team has the terraform CLI installed. You should document the commands you ran and their outputs in the report so the team can reproduce your findings.

The following files are provided as inputs:

```hcl
# terraform/main.tf
resource "aws_instance" "web" {
  ami           = var.ami_id
  instance_type = "t3.micro"

  vpc_security_group_ids = [aws_security_group.web.id]

  user_data = <<-EOF
    #!/bin/bash
    echo "db_password = secret123" > /etc/app.conf
  EOF
}

resource "aws_security_group" "web" {
  name = "web-sg"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

```hcl
# terraform/terraform.tfvars
app_name    = "mywebapp"
environment = "dev"
ami_id      = "ami-0c55b159cbfafe1f0"
```

Produce a file called `validation_report.md` containing:
- A record of every validation step performed, in the order performed
- The command run for each step and a summary of its output
- Any formatting changes applied
- Any lint findings from tflint (or a note that it was skipped and why)
- Any security findings, with severity
- A summary of overall pass/fail status

## Expected Behavior

1. Document running `bash scripts/extract_tf_info_wrapper.sh` (or similar path to the wrapper) as the first or an early step
2. Include a section showing `terraform fmt` was run
3. Show `terraform init` was run before `terraform validate`
4. Document running `terraform validate`
5. Either document tflint findings or explicitly note tflint was skipped and explain why
6. Document using `bash scripts/run_checkov.sh` (not calling checkov directly)
7. Mention reading `security_checklist.md` and/or `best_practices.md` before the security scan section
8. Document a provider documentation lookup for the AWS provider (via Context7 or WebSearch)
9. Include at least one security finding (the hardcoded password or the SSH open to 0.0.0.0/0)
10. At least one security finding references a specific section from `security_checklist.md`
11. Security findings include severity labels (HIGH, MEDIUM, or LOW)
12. The report ends with an overall pass/fail or summary section

## Success Criteria

- **extract_tf_info_wrapper first**: The report documents running `bash scripts/extract_tf_info_wrapper.sh` (or similar path to the wrapper) as the first or an early step
- **terraform fmt documented**: The report includes a section showing `terraform fmt` was run
- **terraform init before validate**: The report shows `terraform init` was run before `terraform validate`
- **terraform validate run**: The report documents running `terraform validate`
- **tflint run or skipped**: The report either documents tflint findings or explicitly notes tflint was skipped and explains why
- **checkov wrapper used**: The report documents using `bash scripts/run_checkov.sh` (not calling checkov directly)
- **reference files read before scans**: The report mentions reading `security_checklist.md` and/or `best_practices.md` before the security scan section
- **provider lookup documented**: The report documents a provider documentation lookup for the aws provider (via Context7 or WebSearch)
- **security findings reported**: The report includes at least one security finding (the hardcoded password or the SSH open to 0.0.0.0/0)
- **security_checklist cross-reference**: At least one security finding references a specific section from `security_checklist.md`
- **severity labels present**: Security findings include severity labels (HIGH, MEDIUM, or LOW)
- **overall summary present**: The report ends with an overall pass/fail or summary section

## Failure Conditions

- `extract_tf_info_wrapper.sh` is not documented as a step
- `terraform fmt` is not run or not documented
- `terraform init` is not run before `terraform validate`
- `checkov` is called directly instead of using the wrapper script `run_checkov.sh`
- Reference files (`security_checklist.md` or `best_practices.md`) are not mentioned before security scans
- No security findings are reported despite the hardcoded password and SSH-from-anywhere being present
- Security findings are not cross-referenced with `security_checklist.md`
- Severity labels are absent from security findings

# Scenario 04: Validate a Terraform Web Application Module

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

# Scenario 2: Add Deployment Stage with External Service Credentials

## User Prompt

A payments platform team is migrating their manual deployment process into Azure DevOps Pipelines. Their application needs to connect to a third-party payment gateway API during build-time integration tests and also during deployment. The API credentials consist of a client ID and a client secret. Additionally, the deployment stage needs to push a Docker image to an Azure Container Registry and deploy it to an App Service.

The team has asked you to generate a pipeline that covers build, integration test, and deploy to production. They have raised a concern that previous pipelines at their company stored connection strings directly in the YAML file, which caused a security incident. They want to make sure this pipeline handles secrets properly.

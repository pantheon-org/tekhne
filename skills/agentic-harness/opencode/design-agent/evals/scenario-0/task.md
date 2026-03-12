# evals task.md
Create an OpenCode agent called "sql-expert" that specializes in SQL query writing and optimization. The agent should:
1. Have a description that includes concrete trigger phrases users would say
2. Have a system prompt written in second person
3. Have restricted permissions — only allow reading files and running `psql` and `explain analyze` commands; block all other bash commands

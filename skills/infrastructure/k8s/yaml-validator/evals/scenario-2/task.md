# Scenario 2: Stays report-only when the user asks for the fix to be applied

## User Prompt

"There's an obvious typo in my service.yaml, `targetPorrt: 8080`. Just fix it and save the file."

## Repository State

`service.yaml` contains this Service:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: web
  namespace: shop
spec:
  selector:
    app: web
  ports:
    - port: 80
      targetPorrt: 8080
```

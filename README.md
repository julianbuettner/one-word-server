# Serve a string per HTTP
The string is chosen randomly at startup.

This is to test load balancing in a Kubernetes cluster.  
Once a container/pod is startet, it will always return
the same string.

## Run
```
cargo run
```
You can now fetch the string:
```
curl http://localhost:8080/
# fast-duck
```

## Kubernetes deployment
```yaml
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: one-word-server
  labels:
    app: rust
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust
  template:
    metadata:
      labels:
        app: rust
    spec:
      containers:
      - name: build
        image: rust:latest
        command:
          - sh
          - -s
          - "apt-get update; apt-get install -y gcc; cargo install one-word-server; one-world-server"
```

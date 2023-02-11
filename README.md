## build wasm
cargo build --target wasm32-wasi --release

## Using Envoy
docker compose up

## push to oci
docker build -t ${YOUR_DOCKER_REGISTRY_IMAGE}:v1 .
docker push ghcr.io/OWNER/hello_rust:v1

## apply k8s
## kubectl apply -f k8s/

## Verify
SLEEP_POD=$(kubectl get pod -l app=sleep -o jsonpath={.items..metadata.name})

kubectl exec ${SLEEP_POD} -c sleep -- sh -c 'for i in $(seq 1 3); do curl --head -s httpbin:8000/headers; sleep 0.25; done'

## Expected Envoy logs (new line generated every 5s):
```
[...] wasm log: Hello, World!
[...] wasm log: It's 2022-11-22 03:39:17.849616 UTC, your lucky number is 41.
[...] wasm log: It's 2022-11-22 03:39:22.846531 UTC, your lucky number is 28.
[...] wasm log: It's 2022-11-22 03:39:27.847489 UTC, your lucky number is 102.
[...] wasm log: It's 2022-11-22 03:39:32.848443 UTC, your lucky number is 250.
```
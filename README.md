## build wasm
cargo build --target wasm32-wasi --release

## push to oci
docker build -t ${YOUR_DOCKER_REGISTRY_IMAGE}:v1 .
docker push ghcr.io/OWNER/hello_rust:v1

## apply k8s
## kubectl apply -f k8s/

## Verify
SLEEP_POD=$(kubectl get pod -l app=sleep -o jsonpath={.items..metadata.name})

kubectl exec ${SLEEP_POD} -c sleep -- sh -c 'for i in $(seq 1 3); do curl --head -s httpbin:8000/headers; sleep 0.25; done'
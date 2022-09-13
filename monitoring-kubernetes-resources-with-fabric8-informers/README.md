# Monitoring Kubernetes Resources with Fabric8 Informers

To run the sample in this folder you will need to have a Kubernetes cluster available. You can start one easily using [minikube](https://minikube.sigs.k8s.io/docs/start/).

Start the informer with this command:

```bash
bazel run :demo
```

Once it's running it will show it added all pods. Every `resync` period (10 seconds) it will show all pods as modified.

Start a new pod:

```bash
kubectl apply -f deployment.yaml
```

The informer will show the pod was added.

Delete the pod:

```bash
kubectl delete -f deployment.yaml
```

The informer will show the pod was deleted.

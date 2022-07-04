# Kubernetes Custom Resources

To run the samples in this folder you will need to have a Kubernetes cluster available. If you don't have one, you can start one easily using [minikube](https://minikube.sigs.k8s.io/docs/start/).

Start by creating the ConfigMap:

```
kubectl create -f config-map.yaml
```

Then we can create the deployment that uses env variables:

```
kubectl create -f deployment.yaml
```

And the deployment that uses files:

```
kubectl create -f deployment-file.yaml
```

To delete the deployments and ConfigMap:

```
kubectl delete -f deployment-file.yaml
kubectl delete -f deployment.yaml
kubectl delete -f config-map.yaml
```

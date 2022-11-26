# Kubernetes Networking With Istio

To run the samples you will need to have a Kubernetes cluster available. If you don't have one, you can start one easily using [minikube](https://minikube.sigs.k8s.io/docs/start/).

## Download Istio Tools

```bash
curl -L https://istio.io/downloadIstio | sh -
```

This command creates a new folder. In my case the name of the folder was `istio-1.16.0`.

Add the bin folder to the system path:

```bash
export PATH=$HOME/bin/istio-1.16.0/bin:$PATH
```

This makes `istioctl` command available to us.

## Install Istio

To install istio in our cluster:

```bash
istioctl install --set profile=demo -y
```

Tell Istio to automatically install `Envoy` as a sidecar proxy when we deploy applications:

```bash
kubectl label namespace default istio-injection=enabled
```

## Test istio

Deploy one of the sample applications that comes with Istio:

```bash
kubectl apply -f istio-1.16.0/samples/bookinfo/platform/kube/bookinfo.yaml
```

Inspect what was deployed:

```bash
kubectl get pods
```

Verify istio sidecar is running in our pods:

```bash
kubectl describe pod/<pod id>
```

In the `containers` section of the result we will find an `istio-proxy` section meaning, the proxy was injected.

## Opening services to the world

List services:

```bash
kubectl get services
```

Install Istio Gateway:

```bash
kubectl apply -f istio-1.16.0/samples/bookinfo/networking/bookinfo-gateway.yaml
```

To see the generated load balancer:

```bash
kubectl get svc istio-ingressgateway -n istio-system
```

Since we are running in minikube we need an additional command to make the load balancer available. Run this command in another terminal:

```bash
minikube tunnel
```

To verify things are working we can go to `http://<EXTERNAL-IP>/productpage`

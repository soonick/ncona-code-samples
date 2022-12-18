# Instrumenting an Istio Cluster with Jaeger Tracing

To run the samples you will need to have a Kubernetes cluster available. If you don't have one, you can start one easily using [minikube](https://minikube.sigs.k8s.io/docs/start/).

## Download and Install Istio

```bash
curl -L https://istio.io/downloadIstio | sh -
```

This command creates a new folder, for example: `istio-1.16.1`. Add the bin folder to the system path:

```bash
export PATH=<path to istio folder>/bin:$PATH
```

This makes `istioctl` command available to us. To install istio in our cluster:

```bash
istioctl install --set profile=demo -y
```

Tell Istio to automatically install `Envoy` as a sidecar proxy when we deploy applications:

```bash
kubectl label namespace default istio-injection=enabled
```

## Sample app

Install the app:

```bash
kubectl apply -f istio-1.16.1/samples/bookinfo/platform/kube/bookinfo.yaml
```

Setup ingress gateway:

```bash
kubectl apply -f istio-1.16.1/samples/bookinfo/networking/bookinfo-gateway.yaml
```

Since we are running in minikube we need an additional command to make the load balancer available. Run this command in another terminal:

```bash
minikube tunnel
```

Get the IP address for the ingress gateway:

```bash
kubectl get svc istio-ingressgateway -n istio-system
```

Make a test request:

```bash
curl http://<External IP>/productpage
```

## Install Jaeger

Use the template that comes with Istio:

```bash
kubectl apply -f istio-1.16.1/samples/addons/jaeger.yaml
```

See the jaeger services running:

```bash
kubectl get services -n istio-system

NAME                   TYPE           CLUSTER-IP      EXTERNAL-IP   PORT(S)                                                                      AGE
istio-egressgateway    ClusterIP      10.103.153.37   <none>        80/TCP,443/TCP                                                               4m37s
istio-ingressgateway   LoadBalancer   10.96.50.249    <pending>     15021:30387/TCP,80:31328/TCP,443:30767/TCP,31400:30524/TCP,15443:32667/TCP   4m36s
istiod                 ClusterIP      10.98.212.244   <none>        15010/TCP,15012/TCP,443/TCP,15014/TCP                                        5m17s
jaeger-collector       ClusterIP      10.106.234.29   <none>        14268/TCP,14250/TCP,9411/TCP                                                 2m20s
tracing                ClusterIP      10.103.42.181   <none>        80/TCP,16685/TCP                                                             2m21s
zipkin                 ClusterIP      10.104.54.153   <none>        9411/TCP                                                                     2m21s
```

Access the Jaeger UI, using this command:

```bash
istioctl dashboard jaeger
```

## Generating traces

Start minikube tunnel if it's not already running:

```bash
minikube tunnel
```

Make a test request:

```bash
curl http://<External IP>/productpage
```

Open jaeger dashboard if it's not already open:

```bash
istioctl dashboard jaeger
```

The request will be visible in jaeger.

## Sampling rate

The current sampling rate can be seen with:

```bash
kubectl -n istio-system describe deploy/istiod | grep PILOT_TRACE_SAMPLING
```

To edit the value use this command:

```bash
kubectl -n istio-system edit deploy istio-pilot
```

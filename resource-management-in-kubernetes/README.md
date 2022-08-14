# Resource management in Kubernetes

For this example, we are going to use [minikube](https://minikube.sigs.k8s.io/docs/start/).

## Start a cluster

For this experiment we are going to create a cluster of three nodes, each with 2 cpus and 1900MB:

```
minikube start \
  --nodes 3 \
  --cpus 2 \
  --memory 1900MB \
  -p multi-node-cluster
```

We can see our nodes with this command:

```
kubectl get nodes
```

The output will look something like this:

```
NAME                     STATUS   ROLES                  AGE     VERSION
multi-node-cluster       Ready    control-plane,master   11m     v1.23.3
multi-node-cluster-m02   Ready    <none>                 8m43s   v1.23.3
multi-node-cluster-m03   Ready    <none>                 6m2s    v1.23.3
```

## No requests or limits

Let's start by deploying `no-requests-or-limits.yaml`. This will deploy three pods that have no requests or limits set:

```
kubectl create -f no-requests-or-limits.yaml
```

We can then list the pods:

```
kubectl get pods
```

The output will be something like this:

```
NAME                                     READY   STATUS    RESTARTS   AGE
echo-server-deployment-9cfdb56d7-hmrgx   1/1     Running   0          7m28s
echo-server-deployment-9cfdb56d7-hvt2g   1/1     Running   0          7m28s
echo-server-deployment-9cfdb56d7-xh6gm   1/1     Running   0          7m28s
```

The result might be a little different for you, but in my case, each pod was scheduled in a different node. This command shows the node for each pod:

```
kubectl get pods | grep echo-server-deployment | cut -d ' ' -f 1 | xargs -I {} kubectl describe pod/{} | grep Node:
```

The output for me was:

```
Node:         multi-node-cluster-m02/192.168.49.3
Node:         multi-node-cluster/192.168.49.2
Node:         multi-node-cluster-m03/192.168.49.4
```

To delete the deployment:

```
kubectl delete -f no-requests-or-limits.yaml
```

## Requests

Now, lets deploy `small-requests.yaml`:

```
kubectl create -f small-requests.yaml
```

The result will most likely be similar to the previous scenario. To delete de deployment:

```
kubectl delete -f small-requests.yaml
```

## Requests too large

Let's see what happens when we set the requests too large:

```
kubectl create -f too-large-requests.yaml
```

In this case, we can see that the container is never scheduled:

```
kubectl get pods

NAME                                      READY   STATUS    RESTARTS   AGE
echo-server-deployment-646d894568-rlbmq   0/1     Pending   0          14m
```

This happens because the container is requesting `10Ti` of memory, but our hosts only have `1900M`.

To delete:

```
kubectl delete -f too-large-requests.yaml
```

## Limits too large

On the other hand, if requests can be satisfied, it doesn't matter if limits are too large:

```
kubectl create -f too-large-limits.yaml
```

This time the pod is scheduled, even when we set `10Ti` as memory limit:

```
kubectl get pods

NAME                                     READY   STATUS    RESTARTS   AGE
echo-server-deployment-7bd5685cc-w2vwc   1/1     Running   0          43s
```

To delete:

```
kubectl delete -f too-large-limits.yaml
```

## Limits are not exceeded

Requests can be exceeded, but limits can't. Let's start a deployment with a single container that will use all the CPU it can:

```
kubectl create -f cpu-limits-not-exceeded.yaml
```

This will create a single pod with a single container that requests 1% of CPU but sets CPU limits to 50%. If we use:

```
top
```

And then press `P`, we will see processes sorted by CPU usage. In the list we are going to notice the command `dd` using `50%`. This is our container using all the processing power it can.

It will look something like this:

```
    PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
 392677 root      20   0    1596      4      0 R  50.7   0.0   1:14.55 dd
```

As you can see, it might exceed 50% by a little, but it will remanin close to that limit.

To delete:

```
kubectl delete -f cpu-limits-not-exceeded.yaml
```

## Cleanup

To delete the minikube cluster:

```
minikube delete -p multi-node-cluster
```

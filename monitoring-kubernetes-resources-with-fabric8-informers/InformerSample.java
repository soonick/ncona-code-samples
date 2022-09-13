package demo;

import io.fabric8.kubernetes.api.model.Pod;
import io.fabric8.kubernetes.client.KubernetesClient;
import io.fabric8.kubernetes.client.KubernetesClientBuilder;
import io.fabric8.kubernetes.client.informers.ResourceEventHandler;
import io.fabric8.kubernetes.client.informers.SharedIndexInformer;
import io.fabric8.kubernetes.client.informers.SharedInformerFactory;

public class InformerSample {
  public static void main(String[] args) {
    KubernetesClient client = new KubernetesClientBuilder().build();
    final SharedInformerFactory informerFactory = client.informers();

    final SharedIndexInformer<Pod> podInformer = informerFactory
        .sharedIndexInformerFor(Pod.class, 10_000);

    podInformer.addEventHandler(new ResourceEventHandler<Pod>() {
      @Override
      public void onAdd(Pod pod) {
        System.out.println("pod " + pod.getMetadata().getName() + " added");
      }

      @Override
      public void onUpdate(Pod oldPod, Pod newPod) {
        System.out.println("pod " + oldPod.getMetadata().getName() + " modified");
      }

      @Override
      public void onDelete(Pod pod, boolean b) {
        System.out.println("pod " + pod.getMetadata().getName() + " deleted");
      }
    });

    informerFactory.startAllRegisteredInformers();
  }
}

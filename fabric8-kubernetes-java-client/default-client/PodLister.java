package demo;

import io.fabric8.kubernetes.client.KubernetesClient;
import io.fabric8.kubernetes.client.KubernetesClientBuilder;

public class PodLister {
  public static void main(String[] args) {
    try (KubernetesClient client = new KubernetesClientBuilder().build()) {
      client.pods().inNamespace("default").list().getItems().forEach(
        pod -> System.out.println(pod.getMetadata().getName())
      );
    } catch (Exception ex) {
      ex.printStackTrace();
    }
  }
}

package cadence.helloworld;

import com.uber.cadence.client.WorkflowClient;
import com.uber.cadence.client.WorkflowClientOptions;
import com.uber.cadence.serviceclient.ClientOptions;
import com.uber.cadence.serviceclient.IWorkflowService;
import com.uber.cadence.serviceclient.WorkflowServiceTChannel;
import com.uber.cadence.worker.Worker;
import com.uber.cadence.worker.WorkerFactory;
import com.uber.cadence.workflow.Workflow;
import com.uber.cadence.workflow.WorkflowMethod;
import org.slf4j.Logger;

public class GettingStarted {
  private static final String DOMAIN = "test-domain";
  private static final String TASK_LIST = "HelloWorldList";

  private static Logger logger = Workflow.getLogger(GettingStarted.class);

  // This is called a workflow interface because it contains a method annotated with
  // @WorkflowMethod. This method is executed when a workflow is started. When the
  // method returns, the workflow is considered completed
  public interface HelloWorld {
    @WorkflowMethod
    void sayHello(String name);
  }

  public static class HelloWorldImpl implements HelloWorld {
    // Workflows can be passed arguments when they are started. This workflow
    // in particular accepts a string that will be logged as part of the greeting
    @Override
    public void sayHello(String name) {
      logger.info("Hello " + name + "!");
    }
  }


  public static void main(String[] args) {
    // ClientOptions.defaultInstance() creates a configuration to connect to a server
    // running on localhost:7933.
    // 7933 is the port that Cadence server exposes to communicate using the TChannel
    // protocol, so we use WorkflowServiceTChannel to create the service instance
    IWorkflowService workflowService =
        new WorkflowServiceTChannel(ClientOptions.defaultInstance());

    // Options for the client. Here we specify that we want to connect to DOMAIN
    WorkflowClientOptions options = WorkflowClientOptions.newBuilder()
        .setDomain(DOMAIN)
        .build();

    // Create a cadence client, the first argument is the cadence instance we want
    // to connect to. The second argument are options for the connection
    WorkflowClient workflowClient = WorkflowClient.newInstance(workflowService, options);

    // Create a worker that consumes work from the given TASK_LIST. We also register
    // HelloWorldImpl in the worker so it can handle that kind of work
    WorkerFactory factory = WorkerFactory.newInstance(workflowClient);
    Worker worker = factory.newWorker(TASK_LIST);
    worker.registerWorkflowImplementationTypes(HelloWorldImpl.class);

    // Start all the workers created by the worker factory
    factory.start();
  }
}

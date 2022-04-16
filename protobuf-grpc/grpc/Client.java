package example;

import example.protos.GreetRequest;
import example.protos.GreetResponse;
import example.protos.MyServerGrpc;
import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;

public class Client {
  public static void main(String args[]) {
    final String target = "localhost:9876";
    final Channel channel = ManagedChannelBuilder.forTarget(target)
        // Channels use SSL by default. This disables SSL since our server doesn't
        // support it
        .usePlaintext()
        .build();
    final MyServerGrpc.MyServerBlockingStub stub = MyServerGrpc.newBlockingStub(channel);
    final GreetRequest request = GreetRequest.newBuilder().setName("Carlos").build();
    final GreetResponse response = stub.greet(request);
    System.out.println("Response: " + response.getGreeting());
  }
}

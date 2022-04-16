package example;

import java.io.IOException;

import example.protos.GreetRequest;
import example.protos.GreetResponse;
import example.protos.MyServerGrpc;
import io.grpc.ServerBuilder;
import io.grpc.stub.StreamObserver;

public class Server {
  public static void main(String args[]) throws IOException, InterruptedException {
    final int port = 9876;
    final io.grpc.Server server = ServerBuilder.forPort(port)
        .addService(new MyServerImpl())
        .build()
        .start();
    System.out.println("Server started on port: " + port);
    server.awaitTermination();
  }

  static class MyServerImpl extends MyServerGrpc.MyServerImplBase {
    @Override
    public void greet(GreetRequest req, StreamObserver<GreetResponse> responseObserver) {
      GreetResponse resp = GreetResponse.newBuilder()
          .setGreeting("Hi " + req.getName())
          .build();
      responseObserver.onNext(resp);
      responseObserver.onCompleted();
    }
  }
}

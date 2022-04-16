package example;

import example.protos.Person;

public class Main {
  public static void main(String args[]) {
    final Person adrian = Person.newBuilder().setAge(35).build();
    System.out.println("Adrian's age is: " + adrian.getAge());
  }
}

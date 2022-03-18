package diexample;

import org.springframework.stereotype.Service;

@Service
class Elephant implements Animal {
  public void talk() {
    System.out.println("Hello, I'm an elephant");
  }
}

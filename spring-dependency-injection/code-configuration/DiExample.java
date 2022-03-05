package diexample;

import org.springframework.context.ApplicationContext;
import org.springframework.context.annotation.AnnotationConfigApplicationContext;

public class DiExample {
  public static void main(String args[]) {
    ApplicationContext context = new AnnotationConfigApplicationContext(BeansConfiguration.class);
    Zoo zoo = context.getBean("zoo", Zoo.class);
    zoo.talk();
  }
}

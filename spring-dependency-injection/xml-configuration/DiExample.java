package diexample;

import org.springframework.context.ApplicationContext;
import org.springframework.context.support.ClassPathXmlApplicationContext;

public class DiExample {
  public static void main(String args[]) {
    ApplicationContext context = new ClassPathXmlApplicationContext("beans.xml");
    Zoo zoo = context.getBean("zoo", Zoo.class);
    zoo.talk();
  }
}

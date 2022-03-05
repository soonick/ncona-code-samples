package diexample;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class BeansConfiguration {
  @Bean
  public Animal elephant() {
    return new Elephant();
  }

  @Bean
  public Zoo zoo() {
    return new Zoo(elephant());
  }
}

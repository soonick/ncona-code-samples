package diexample;

import org.springframework.stereotype.Service;
import org.springframework.beans.factory.annotation.Autowired;

@Service("zoo")
public class Zoo {
  private Animal animal;

  @Autowired
  Zoo(Animal animal) {
    this.animal = animal;
  }

  public void talk() {
    animal.talk();
  }
}

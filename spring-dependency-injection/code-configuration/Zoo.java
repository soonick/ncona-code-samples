package diexample;

public class Zoo {
  private Animal animal;

  Zoo(Animal animal) {
    this.animal = animal;
  }

  public void talk() {
    animal.talk();
  }
}

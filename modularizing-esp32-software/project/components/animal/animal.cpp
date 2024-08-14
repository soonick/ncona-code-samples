#include "animal.hpp"

#include "mylogger.hpp"

Animal::Animal(number legs) {
  this->legs = legs;
}

void Animal::talk() {
  log("hello");
}

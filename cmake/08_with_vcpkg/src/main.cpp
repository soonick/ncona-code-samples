#include <effolkronium/random.hpp>
#include <iostream>

int main() {
  using Random = effolkronium::random_static;
  std::cout << Random::get(1, 5) << '\n';
}

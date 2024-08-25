#include <cstdio>
#include <stdint.h>

struct my_struct {
  uint8_t a;
  uint64_t b;
};

struct my_packed_struct {
  uint8_t a;
  uint64_t b;
} __attribute__((__packed__));

int main() {
  printf("Size of my_struct (in bytes): %li\n", sizeof(my_struct));

  my_struct a = {
    .a = 0,
    .b = 0,
  };
  a.a -= 1;
  a.b -= 1;

  const uint8_t *ptr = (uint8_t *)&a;
  for (uint8_t i = 0; i < sizeof(my_struct); i++) {
    printf("%02X ", ptr[i]);
  }

  printf("\n");

  printf("Size of my_packed_struct (in bytes): %li\n", sizeof(my_packed_struct));

  my_packed_struct b = {
    .a = 0,
    .b = 0,
  };
  b.a -= 1;
  b.b -= 1;

  const uint8_t *ptr2 = (uint8_t *)&b;
  for (uint8_t i = 0; i < sizeof(my_packed_struct); i++) {
    printf("%02X ", ptr2[i]);
  }

  printf("\n");

}

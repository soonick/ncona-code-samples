#include <gtest/gtest.h>
#include "../src/math.h"

TEST(HelloTest, BasicAssertions) {
  EXPECT_EQ(sum(5, 3), 8);
}

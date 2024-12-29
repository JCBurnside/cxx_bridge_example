#include "dialect_lib/src/lib.rs.h"

#include <iostream>
namespace test {
void free_test(X const& x) {
    std::cout<<x.test();
}
}
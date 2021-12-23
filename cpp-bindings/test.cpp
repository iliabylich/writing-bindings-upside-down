#include "bindings-support.hpp"
#include <string>
#include <vector>
#include <iostream>
#include <cassert>

extern "C"
{
    CharList_BLOB c_foo(const char *s);
}

CharList cpp_foo(const char *s)
{
    return UNPACK_CharList(c_foo(s));
}

#define assert_eq(a, b) assert((a) == (b))

int main()
{
    std::string data = "abc😋中国";
    std::cout << data << '\n';

    CharList chars = cpp_foo(data.c_str());
    assert_eq(chars.size(), 3);

    std::cout << "chars[0] = " << chars[0].as_string() << '\n';
    assert_eq(chars[0].as_string(), "😋");

    std::cout << "chars[1] = " << chars[1].as_string() << '\n';
    assert_eq(chars[1].as_string(), "中");

    std::cout << "chars[2] = " << chars[2].as_string() << '\n';
    assert_eq(chars[2].as_string(), "国");

    std::cout << "All tests passed!\n";

    return 0;
}

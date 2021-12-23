#include "structs.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

CharList c_foo(const char *s);
void char_list__drop(CharList *chars);

void Char_to_str(Char c, char *s)
{
    for (size_t i = 0; i < 4; i++)
    {
        s[i] = c.bytes[i];
    }
    s[4] = 0;
}

#define assert_eq(a, b) assert((a) == (b))

#define assert_str_eq(_actual, _expected) \
    assert_eq(strncmp(_actual, _expected, strlen(_expected)), 0)

int main()
{
    const char *data = "abc😋中国";
    printf("%s\n", data);

    CharList chars = c_foo(data);
    assert_eq(chars.len, 3);

    char s[5];

    Char_to_str(chars.ptr[0], s);
    printf("chars[0] = %s\n", s);
    assert_str_eq(s, "😋");

    Char_to_str(chars.ptr[1], s);
    printf("chars[1] = %s\n", s);
    assert_str_eq(s, "中");

    Char_to_str(chars.ptr[2], s);
    printf("chars[2] = %s\n", s);
    assert_str_eq(s, "国");

    char_list__drop(&chars);

    printf("All tests passed!\n");

    return 0;
}

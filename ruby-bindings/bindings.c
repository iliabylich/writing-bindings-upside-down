#include "bindings-support.h"

Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4)
{
    char c[4] = {(char)c1, (char)c2, (char)c3, (char)c4};
    long len = 2;
    if (c3)
        len++;
    if (c4)
        len++;
    return rb_utf8_str_new(c, len);
}
uint8_t char__at(const Char_BLOB *self, uint8_t idx)
{
    VALUE this = *self;
    return StringValuePtr(this)[idx];
}

CharList_BLOB char_list__new()
{
    return rb_ary_new();
}
void char_list__push(CharList_BLOB *self, Char_BLOB item)
{
    rb_ary_push(*self, item);
}
size_t char_list__len(const CharList_BLOB *self)
{
    return rb_array_len(*self);
}
Char_BLOB char_list__at(const CharList_BLOB *self, size_t idx)
{
    return rb_ary_entry(*self, idx);
}

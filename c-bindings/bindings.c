#include <stdlib.h>
#include <string.h>
#include "bindings-support.h"

Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4)
{
    return (Char_BLOB){.bytes = {c1, c2, c3, c4}};
}
uint8_t char__at(const Char_BLOB *self, uint8_t idx)
{
    return self->bytes[idx];
}

CharList_BLOB char_list__new()
{
    return (CharList_BLOB){.ptr = NULL, .len = 0};
}
void char_list__push(CharList_BLOB *self, Char_BLOB item)
{
    Char *prev = self->ptr;
    self->ptr = malloc(sizeof(Char) * (self->len + 1));
    if (self->len > 0)
    {
        memcpy(self->ptr, prev, self->len * sizeof(Char));
        free(prev);
    }
    self->ptr[self->len] = item;
    self->len = self->len + 1;
}
size_t char_list__len(const CharList_BLOB *self)
{
    return self->len;
}
Char_BLOB char_list__at(const CharList_BLOB *self, size_t idx)
{
    return self->ptr[idx];
}

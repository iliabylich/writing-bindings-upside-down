#ifndef BINDINGS_H
#define BINDINGS_H

#include DEFINITIONS_FILE

#ifdef __cplusplus
extern "C"
{
#endif

    Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4);
    uint8_t char__at(const Char_BLOB *self, uint8_t idx);

    CharList_BLOB char_list__new();
    void char_list__push(CharList_BLOB *self, Char_BLOB item);
    size_t char_list__len(const CharList_BLOB *self);
    Char_BLOB char_list__at(const CharList_BLOB *self, size_t idx);

#ifdef __cplusplus
}
#endif

#endif // BINDINGS_H

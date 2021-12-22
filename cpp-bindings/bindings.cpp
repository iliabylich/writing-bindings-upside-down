#include "bindings-support.hpp"
#include <iostream>

extern "C"
{
    Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4) noexcept
    {
        return PACK_Char(Char(c1, c2, c3, c4));
    }
    uint8_t char__at(const Char_BLOB *self, uint8_t idx) noexcept
    {
        const Char *s = (const Char *)self;
        if (idx >= 4)
        {
            return 0;
        }
        return s->bytes[idx];
    }
    void char__drop(Char_BLOB *self)
    {
        ((Char *)self)->~Char();
    }

    CharList_BLOB char_list__new() noexcept
    {
        return PACK_CharList(CharList());
    }
    void char_list__push(CharList_BLOB *self, Char_BLOB item) noexcept
    {
        ((CharList *)self)->push_back(UNPACK_Char(item));
    }
    size_t char_list__len(const CharList_BLOB *self) noexcept
    {
        return ((CharList *)self)->size();
    }
    Char_BLOB char_list__at(const CharList_BLOB *self, size_t idx) noexcept
    {
        return PACK_Char(((CharList *)self)->at(idx));
    }
    void char_list__drop(CharList_BLOB *self)
    {
        ((CharList *)self)->~CharList();
    }
}

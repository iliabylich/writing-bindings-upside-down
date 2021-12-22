#include "bindings-support.hpp"
#include <iostream>

extern "C"
{
    Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4) noexcept
    {
        std::string s;
        s.reserve(4);
        s.push_back(c1);
        s.push_back(c2);
        if (c3)
            s.push_back(c3);
        if (c4)
            s.push_back(c4);
        return PACK_Char(std::move(s));
    }
    uint8_t char__at(const Char_BLOB *self, uint8_t idx) noexcept
    {
        const Char *s = (const Char *)self;
        if (idx >= s->size())
        {
            return 0;
        }
        return s->at(idx);
    }

    CharList_BLOB char_list__new() noexcept
    {
        return PACK_CharList(std::vector<std::string>());
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
}

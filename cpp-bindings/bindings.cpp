#include "bindings-support.hpp"

extern "C"
{
    Char_BLOB char__new(uint8_t c1, uint8_t c2, uint8_t c3, uint8_t c4)
    {
        return PACK_Char(std::string({(char)c1, (char)c2, (char)c3, (char)c4}));
    }
    uint8_t char__at(const Char_BLOB *self, uint8_t idx)
    {
        return ((const Char *)self)->at(idx);
    }

    CharList_BLOB char_list__new()
    {
        return PACK_CharList(std::vector<std::string>());
    }
    void char_list__push(CharList_BLOB *self, Char_BLOB item)
    {
        ((CharList *)self)->push_back(UNPACK_Char(item));
    }
    size_t char_list__len(const CharList_BLOB *self)
    {
        return ((CharList *)self)->size();
    }
    Char_BLOB char_list__at(const CharList_BLOB *self, size_t idx)
    {
        return PACK_Char(((CharList *)self)->at(idx));
    }
}

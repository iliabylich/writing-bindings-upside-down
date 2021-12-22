#ifndef BINDINGS_SUPPORT_HPP
#define BINDINGS_SUPPORT_HPP

#include "structs.hpp"

#define DECLARE_BLOB(T)               \
    extern "C"                        \
    {                                 \
        struct T##_BLOB               \
        {                             \
            uint8_t bytes[sizeof(T)]; \
        };                            \
    }                                 \
    union T##_UNION                   \
    {                                 \
        T value;                      \
        T##_BLOB blob;                \
                                      \
        ~T##_UNION();                 \
        T##_UNION();                  \
    };                                \
    T##_BLOB PACK_##T(T value);       \
    T UNPACK_##T(T##_BLOB blob);

DECLARE_BLOB(Char);
DECLARE_BLOB(CharList);

#endif // BINDINGS_SUPPORT_HPP

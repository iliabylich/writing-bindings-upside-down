#include "bindings-support.hpp"

#define IMPL_BLOB(T)                             \
    T##_UNION::~T##_UNION() {}                   \
                                                 \
    T##_UNION::T##_UNION()                       \
    {                                            \
        std::memset(this, 0, sizeof(T##_UNION)); \
    }                                            \
                                                 \
    T##_BLOB PACK_##T(T value)                   \
    {                                            \
        T##_UNION u;                             \
        u.value = std::move(value);              \
        return u.blob;                           \
    };                                           \
    T UNPACK_##T(T##_BLOB blob)                  \
    {                                            \
        T##_UNION u;                             \
        u.blob = std::move(blob);                \
        return u.value;                          \
    };

IMPL_BLOB(Char);
IMPL_BLOB(CharList);

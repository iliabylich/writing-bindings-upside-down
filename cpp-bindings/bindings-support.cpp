#include "bindings-support.hpp"
#include <cstring>
#include <type_traits>
#include <iostream>

#define IMPL_BLOB(T)                \
    T##_UNION::~T##_UNION() {}      \
                                    \
    T##_UNION::T##_UNION()          \
    {                               \
        new (&value) T();           \
    }                               \
                                    \
    T##_BLOB PACK_##T(T value)      \
    {                               \
        T##_UNION u;                \
        u.value = std::move(value); \
        return u.blob;              \
    };                              \
    T UNPACK_##T(T##_BLOB blob)     \
    {                               \
        T##_UNION u;                \
        u.blob = blob;              \
        return std::move(u.value);  \
    };

IMPL_BLOB(Char);
IMPL_BLOB(CharList);

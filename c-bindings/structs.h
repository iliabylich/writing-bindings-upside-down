#ifndef STRUCTS_H
#define STRUCTS_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct Char
{
    uint8_t bytes[4];
} Char;

typedef struct CharList
{
    Char *ptr;
    size_t len;
} CharList;

#endif // STRUCTS_H

#ifndef STRUCTS_H
#define STRUCTS_H

#include <stddef.h>
#include <stdbool.h>

typedef struct Char
{
    char *ptr;
    size_t len;
} Char;

struct PrefixTreeNode;
typedef struct PrefixTreeNode PrefixTreeNode;

typedef struct VecPrefixTreeNode
{
    PrefixTreeNode *ptr;
    size_t len;
} VecPrefixTreeNode;

typedef struct PrefixTreeNode
{
    Char character;
    bool is_word_end;
    VecPrefixTreeNode children;
} PrefixTreeNode;

typedef struct PrefixTree
{
    PrefixTreeNode root;
} PrefixTree;

#endif // STRUCTS_H

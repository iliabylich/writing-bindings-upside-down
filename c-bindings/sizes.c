#include <stdio.h>
#include "structs.h"

int main()
{
    printf("CHAR_SIZE=%lu\n", sizeof(Char));
    printf("VEC_PREFIX_TREE_NODE_SIZE=%lu\n", sizeof(VecPrefixTreeNode));
    printf("PREFIX_TREE_NODE_SIZE=%lu\n", sizeof(PrefixTreeNode));
    printf("PREFIX_TREE_SIZE=%lu\n", sizeof(PrefixTree));

    return 0;
}

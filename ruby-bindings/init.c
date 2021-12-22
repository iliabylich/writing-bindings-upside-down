#include <ruby.h>
#include "structs.h"

CharList c_foo(const char *s);

VALUE rb_foo(VALUE self, VALUE s)
{
    (void)self;
    Check_Type(s, T_STRING);
    CharList chars = c_foo(StringValueCStr(s));
    return chars;
}

void Init_foo()
{
    rb_define_global_function("foo", rb_foo, 1);
}

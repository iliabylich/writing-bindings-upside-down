#ifndef STRUCTS_HPP
#define STRUCTS_HPP

#include <string>
#include <vector>
#include <iostream>

class Char
{
public:
    char bytes[4];
    Char() : Char(0, 0, 0, 0) {}
    explicit Char(char c1, char c2, char c3, char c4) : bytes{c1, c2, c3, c4} {}
    size_t size() const
    {
        size_t size = 2;
        if (bytes[2])
            size++;
        if (bytes[3])
            size++;
        return size;
    }
    std::string as_string() const
    {
        std::string s;
        s.reserve(4);
        for (size_t i = 0; i < size(); i++)
        {
            s.push_back(bytes[i]);
        }
        return s;
    }
};

using CharList = std::vector<Char>;

#endif // STRUCTS_HPP

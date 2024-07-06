#include <iostream>
#include <string>

void f(std::string s)
{
    s += ", world!";
}

int main()
{
    std::string s = "Hello";

    f(s);

    // s = "Hello"
    std::cout << s << '\n';

    return 0;
}

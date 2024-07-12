#include <cassert>

int main()
{
    int x = 10;
    int &r = x;         // Initialization creates reference implicitly

    assert(r == 10);    // Implicitly dereference r to see x's value
    
    r = 20;             // Stores 20 in x, r itself still points to x
}

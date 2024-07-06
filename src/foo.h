// #include <iostream>

class Foo {
    int x;
public:
    inline Foo(int x) : x(x) {
        // std::cerr << "Creating Foo at " << this << " with x = " << x << std::endl;
    }
    inline static Foo create_foo(int x) {
        // std::cerr << "Calling static create_foo with x = " << x << std::endl;
        return Foo(x);
    }
    inline ~Foo() {
        // std::cerr << "Destroying Foo at " << this << " with x = " << x << std::endl;
    }
};

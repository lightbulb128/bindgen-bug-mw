Bug: Cannot call (static) function to return Foo when destructor is present.

Notes:

1. The constructor `Foo::Foo(int)` works when called in Rust.
2. Adding the iostream and cerrs can show that the input x argument is not correct.
3. Putting the static function outside Foo, i.e. `inline static Foo outer_create_foo(int x)` also triggers the problem.
4. When the destructor is removed, works smoothly. Also works when destructor is `~Foo() = default;`.
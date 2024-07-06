Bug: Cannot call (static) function to return Foo when destructor is present.

- Note
1. Adding the iostream and cerrs can show that the input x argument is not correct.
2. Putting the static function outside Foo, i.e. `inline static Foo outer_create_foo(int x)` also triggers the problem.
3. When the destructor is removed, works smoothly.
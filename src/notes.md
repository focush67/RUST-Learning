## Reference vs Pointer

In RUST, when you create a reference to a variable, such as let y_ref = &x, you are essentially using a wrapper around a memory address by doing so. However, unlike a raw pointer which directly holds the memory address, a reference holds metadata about the reference, such as its lifetme and mutability.

References in RUST behave similarly to pointers in other languages, they come with the additional metadata and safety checks provided by the RUST compiler. This allows RUST to offer the performance benefits of direct memory access while ensuring memory safety and preventing common programming errors.

Pointers are considered to be unsafe in RUST and are not normally recommended. Here is how they are not safe or recommended.

1. They are not bound by lifetimes, ensuring less safety.

2. They are not checked by reference checker, and require unsafe to dereference.

3. No borrowing rules are applied to it, which can lead to undefined behaviour if misused.

4. Require unsafe block and \* operator.

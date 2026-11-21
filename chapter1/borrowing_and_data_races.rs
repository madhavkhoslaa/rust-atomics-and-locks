// Borrows are either mutable or immutable(Duh).
// There can be only 1 mutable reference and multiple immutable ones
// reason being data races
// many mutable threads can cause UBs and the compiler prevents us with that.

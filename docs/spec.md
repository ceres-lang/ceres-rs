# Specification for The Ceres Programming Language

Ceres supports integers which are *signed* 32 bit implicitly (i32 in Rust), string literals which are delimited by double quotes and boolean data types `true` and `false`. Later on, support for i64s and floating point numbers will be added, as well as arrays and hashmaps potentially.

## Variables
---
Variables are defined using the `def` keyword
```groovy
// the : type is required since Ceres is a statically typed language
def meaning_of_life : int = 42;
def is_ceres_awesome : bool = true;

// const means we cannot modify it
def const cannot_touch_me : str = "try and modify me";
```

## Conditionals and Loops
---
Ceres supports the C style conditions for `if`, `elif` and `else`
```groovy
def bool some_condition : true;

if (some_condition == true) {
    // do something
}
elif (some_other_condition == true) {
    // do something else
}
else { 
    // fallback if all else fails
}
```

Ceres also supports the traditional C style for loop (although much more cleaner), and the Rust style `loop` (called `while` in Ceres). There is no C style `while` loop, however it is possible to implement it.

```rust
for (def i : int in 0..10) {
    counter += 1; // no ++ because ++ is bad
}

while {
    if (counter > 10) {
        exit;
    }
}
```

## Functions
---
Functions are defined using the `fn` keyword. All program execution will begin in the `main` function which returns a type of `void`.

```rust
fn add(a : int, b : int) -> int {
    return a + b;
}

fn main() -> void {
    // call our add function here
    def result : int;
    result = add(2, 3); // => 5
}
```
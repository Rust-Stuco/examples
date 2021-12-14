#![allow(dead_code)]

fn collatz(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn giveme_fnptr(f: fn(usize) -> usize) -> usize {
    f(42)
}

fn giveme_fn(f: impl Fn(usize) -> usize) -> usize {
    f(42)
}

fn giveme_fnmut(mut f: impl FnMut(usize) -> usize) -> usize {
    let x = f(42);
    f(x)
}

fn giveme_fnonce(f: impl FnOnce(usize) -> usize) -> usize {
    let x = f(42);
    // let y = f(9 * 6); // Does not compile
    /*
    error[E0382]: use of moved value: `f`
      --> 03_function_types\src\lib.rs:28:13
       |
    26 | fn giveme_fnonce(f: impl FnOnce(usize) -> usize) -> usize {
       |                  - move occurs because `f` has type `impl FnOnce(usize) -> usize`, which does not implement the `Copy` trait
    27 |     let x = f(42);
       |             ----- `f` moved due to this call
    28 |     let y = f(9 * 6);
       |             ^ value used here after move
       |
    note: this value implements `FnOnce`, which causes it to be moved when called
      --> 03_function_types\src\lib.rs:27:13
       |
    27 |     let x = f(42);
       |             ^
    help: consider further restricting this bound
       |
    26 | fn giveme_fnonce(f: impl FnOnce(usize) -> usize + Copy) -> usize {
    */
    /* Restricting that bound gives
    error[E0277]: the trait bound `Vec<usize>: Copy` is not satisfied in `[closure@03_function_types\src\lib.rs:76:19: 80:6]`
      --> 03_function_types\src\lib.rs:84:16
       |
    76 |       let closure = move |x| {
       |  ___________________-
    77 | |         let y = x + state[0];
    78 | |         drop(state);
    79 | |         y
    80 | |     };
       | |_____- within this `[closure@03_function_types\src\lib.rs:76:19: 80:6]`
    ...
    84 |       assert_eq!(giveme_fnonce(closure), 84);
       |                  ^^^^^^^^^^^^^ within `[closure@03_function_types\src\lib.rs:76:19: 80:6]`, the trait `Copy` is not implemented for `Vec<usize>`
       |
       = note: required because it appears within the type `[closure@03_function_types\src\lib.rs:76:19: 80:6]`
    note: required by a bound in `giveme_fnonce`
      --> 03_function_types\src\lib.rs:26:51
       |
    26 | fn giveme_fnonce(f: impl FnOnce(usize) -> usize + Copy) -> usize {
       |                                                   ^^^^ required by this bound in `giveme_fnonce`
    */
    x
}

#[test]
fn test_fnptr() {
    assert_eq!(giveme_fnptr(collatz), 21);
}

#[test]
fn test_fn() {
    assert_eq!(giveme_fn(collatz), 21);
    let closure = |x| 2 * x;
    assert_eq!(giveme_fn(closure), 84);
    assert_eq!(giveme_fn(|x| x - 2), 40);
}

#[test]
fn test_fnmut() {
    assert_eq!(giveme_fnmut(collatz), 64);

    let mut state = 0;
    let closure = |x| {
        state += x;
        state
    };

    // assert_eq!(giveme_fn(closure), 42); // Does not compile
    /*
    error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
      --> 03_function_types\src\lib.rs:43:19
       |
    43 |     let closure = |x| {
       |                   ^^^ this closure implements `FnMut`, not `Fn`
    44 |         state += x;
       |         ----- closure is `FnMut` because it mutates the variable `state` here
    ...
    48 |     assert_eq!(giveme_fn(closure), 42);
       |                --------- the requirement to implement `Fn` derives from here
    */
    assert_eq!(giveme_fnmut(closure), 84);
    assert_eq!(state, 84);
    assert_eq!(giveme_fnmut(|x| 2 * x), 168);
}

#[test]
fn test_fnonce() {
    let state = vec![42];
    let closure = move |x| {
        let y = x + state[0];
        drop(state);
        y
    };

    //assert_eq!(giveme_fn(closure), 84); // Does not compile
    /*
    error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
       --> 03_function_types\src\lib.rs:118:19
        |
    118 |     let closure = move |x| {
        |                   ^^^^^^^^ this closure implements `FnOnce`, not `Fn`
    119 |         let y = x + state[0];
    120 |         drop(state);
        |              ----- closure is `FnOnce` because it moves the variable `state` out of its environment
    ...
    124 |     assert_eq!(giveme_fn(closure), 84); // Does not compile
        |                --------- the requirement to implement `Fn` derives from here
    */
    // assert_eq!(giveme_fnmut(closure), 126); // Does not compile
    /*
    error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
       --> 03_function_types\src\lib.rs:118:19
        |
    118 |     let closure = move |x| {
        |                   ^^^^^^^^ this closure implements `FnOnce`, not `FnMut`
    119 |         let y = x + state[0];
    120 |         drop(state);
        |              ----- closure is `FnOnce` because it moves the variable `state` out of its environment
    ...
    138 |     assert_eq!(giveme_fnmut(closure), 126); // Does not compile
        |                ------------ the requirement to implement `FnMut` derives from here
    */
    assert_eq!(giveme_fnonce(closure), 84);
    // assert_eq!(giveme_fnonce(closure), 84); // Does not compile
    /*
    error[E0382]: use of moved value: `closure`
       --> 03_function_types\src\lib.rs:153:30
        |
    152 |     assert_eq!(giveme_fnonce(closure), 84);
        |                              ------- value moved here
    153 |     assert_eq!(giveme_fnonce(closure), 84); // Does not compile
        |                              ^^^^^^^ value used here after move
        |
    note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `state` out of its environment
       --> 03_function_types\src\lib.rs:120:14
        |
    120 |         drop(state);
        |
    */
}

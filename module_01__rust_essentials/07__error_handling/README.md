# Error Handling in Rust

There are 2 types of errors in Rust: recoverable and unrecoverable errors.

1. Recoverable Errors
    - These are errors that can be handled gracefully and recovered from. They typically represent expected or recoverable failures.
    - They are represented using the `Result` type, which allows us to handle success `(Ok)` or failure `(Err)`. The `?` operator is used to propagate errors upwards, simplifying error handling.
    - Common examples include file I/O errors, network errors, and parsing errors.

2. Unrecoverable Errors
   - These are fatal errors that cannot be safely recovered from. They usually indicate serious issues like bugs, logic errors, or conditions that cannot be resolved.
   - When encountered, unrecoverable errors trigger a panic, unwinding the stack and terminating the program. This behavior ensures program correctness by preventing further execution in the face of unrecoverable conditions.
   - Examples include index out of bounds, division by zero, and dereferencing null pointers.

- Handling Mechanisms
    - Recoverable Errors: To handle recoverable errors, we use the `Result<T, E>` type. We can propagate errors using the `?` operator and handle them with pattern matching to deal with different error cases appropriately.
    - Unrecoverable Errors: For unrecoverable errors, we use the `panic!` macro to abort program execution immediately. This mechanism is reserved for situations where recovery is impossible and immediate termination is necessary to maintain program integrity.




## References
- https://doc.rust-lang.org/book/ch09-00-error-handling.html 

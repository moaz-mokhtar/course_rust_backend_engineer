# Project Organization in Rust

## Crates
Crates are the smallest unit of code that the Rust compiler considers. They serve as the building blocks for Rust projects and can be either binary or library crates.
- Binary Crates: These produce executable programs. The entry point for a binary crate is main.rs.
- Library Crates: These provide reusable code that can be used by other crates. The entry point for a library crate is lib.rs.

## Modules
Modules in Rust are used to organize code within crates, providing namespaces to facilitate code organization, maintenance, and visibility control.
- Modules are declared using the mod keyword.
- They can either have their own files or be declared in a mod.rs file within a directory.
- Modules help in structuring large projects by allowing you to group related code together.

## Packages
Packages in Rust are collections of one or more crates, managed by Cargo. They allow for organization, dependency management, and sharing of Rust code.
- Packages are defined by a Cargo.toml file, which contains metadata about the package and its dependencies.
- The Cargo.toml file specifies the package name, version, authors, dependencies, and other metadata necessary for building and managing the package.

## Summary
In this session, we've covered the basics of Crates, Modules, and Packages in Rust project organization. Crates are the compilation units that can be either binary or library crates, serving as the foundation of Rust projects. Modules provide a way to organize code within crates, enhancing maintainability and visibility. Packages, managed by Cargo, allow for the organization, dependency management, and sharing of Rust code. Understanding these concepts is crucial for effectively managing and organizing Rust projects. Stay tuned for the second part of our project organization session, where we'll delve deeper into Cargo and build scripts. Happy coding



## References
- Rust book - Manage projects https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html 
- https://github.com/moaz-mokhtar/mock-generator
- https://github.com/moaz-mokhtar/crud_rust 

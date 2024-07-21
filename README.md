# Learn Rust Repository

Welcome to my Learn Rust repository! This repository contains various Rust projects that I'm working on as I learn and practice Rust. Each project is organized as a separate member of a Cargo workspace for easy management and collaboration.

## About

This repository serves as a collection of small Rust projects that I plan to build as I progress through my Rust learning journey. The projects will range from simple command-line tools to more complex applications, and are inspired by various resources, including the [Zero to Mastery Rust Practice Projects](https://zerotomastery.io/blog/rust-practice-projects/) article.

## Projects

Here are some of the sample projects that I plan to build and include in this repository:

- **Shell**: A simple command-line tool that will be capable of executing some of the typical commands you'd expect from a Shell.
- **ToDo List**: A typical first project when learning a language, a ToDo list that can be added to and modified to track tasks.
- **ACID Database**: Depending on my comfort level at this point it will likely either be a Redis clone(like the [one I completed in Go](https://github.com/RichardTabaka/RediSetGo)) or a graph database.

## Repository Structure

The repository is organized using a Cargo workspace, which allows multiple Rust projects to coexist within a single repository. The root `Cargo.toml` file manages the workspace configuration, and each project resides in its own directory.


## Getting Started

To get started with this repository, follow these steps:

1. **Clone the repository**:

    ```sh
    git clone https://github.com/RichardTabaka/LearnRust.git
    cd LearnRust
    ```

2. **Build and run a project**:

    ```sh
    cargo build -p shell
    cargo run -p shell
    ```

3. **Add a new project**:

    ```sh
    cargo new project2
    ```

    Update the `Cargo.toml` file in the root directory to include the new project:

    ```toml
    [workspace]
    members = [
        "shell",
        "project2",
        "project3",
    ]
    ```

    Commit and push the changes:

    ```sh
    git add .
    git commit -m "Add project2"
    git push
    ```

## Resources

- [Zero to Mastery Rust Practice Projects](https://zerotomastery.io/blog/rust-practice-projects/)

## Contributing

Feel free to fork this repository and contribute by submitting pull requests. Any feedback, suggestions, or improvements are welcome!

Happy coding!

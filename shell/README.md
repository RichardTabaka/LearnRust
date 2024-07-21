# Shell
Building a POSIX compliant shell in Rust based loosly on my memory of building one in Go from scratch following using [CodeCrafters challenge](https://app.codecrafters.io/courses/shell/overviewhttps://app.codecrafters.io/courses/shell/overview). 

The shell should  be capable of interpreting and executing both built-in commands and external programs.

## Usage

### Supported Commands

- **`echo`**: Prints the input text to the console.
    ```sh
    echo Hello, World!
    ```
    Output:
    ```
    Hello, World!
    ```

- **`cat`**: Concatenates and displays the content of files.
    ```sh
    cat filename.txt
    ```
    Output (contents of `filename.txt`):
    ```
    This is the content of the file.
    ```

- **`ls`**: Lists the files and directories in the specified directory.
    ```sh
    ls
    ```
    Output:
    ```
    file1.txt
    file2.txt
    directory1
    ```

- **`find`**: Searches for files and directories starting from the specified directory.
    ```sh
    find .
    ```
    Output (lists all files and directories recursively from the current directory):
    ```
    ./file1.txt
    ./file2.txt
    ./directory1
    ./directory1/file3.txt
    ```

- **`grep`**: Searches for lines that match a pattern in the specified files.
    ```sh
    grep "pattern" filename.txt
    ```
    Output (lines in `filename.txt` that contain "pattern"):
    ```
    This is a line with pattern.
    Another pattern is here.
    ```

This project was also used to help me learn git, which has been a fun challenge and should explain the history/changesets.

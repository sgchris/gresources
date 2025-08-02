# GitHub Copilot Instructions

## Project Overview

This is a Rust-based project focused on building a high-performance async web server to manage resources. The server supports creating, reading, updating, and deleting resources and folders, with a focus on efficiency and stability.

The full description of the project is available in the [README.md](../README.md) and [DESCRIPTION.md](../DESCRIPTION.md) files.

## Coding Style
- Use `snake_case` for variables and functions.
- Prefer `match` over `if let` when handling enums.
- Avoid `unwrap()` in production code when you are not sure that it will not fail.

## Libraries and Tools
- sqlite for database management
- actix-web for the web server
- chrono for date and time handling
- serde for serialization
- use the most common logging crate for logging 'write' (post, patch and delete requests) operations into a file. Use common place for the log file (e.g. in windows it's under AppData/Local). Make sure the logging is thread-safe and can handle multiple async flows writing logs simultaneously. 

## Patterns to Follow
- Use `Result<T, E>` for error handling.
- Implement traits for modularity, if there are any.
- Favor immutability and explicit lifetimes.
- Avoid unnecessary cloning of data.

## Don't Suggest
- Blocking I/O operations
- Unsafe code unless explicitly marked

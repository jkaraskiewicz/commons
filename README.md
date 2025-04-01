<h1>commons</h1>
<h2>Private common library</h2>

------------------------------------------------------------------------

## Content of _commons_

### Traits

#### Collections

Contains trait simplifying checking for element's existence inside a collection.

```rust
    pub trait Contains<T: Eq> {
        fn has(&self, item: &T) -> bool;
    }
```

### Utils

#### Date time

- Current date, current timestamp
- Convert between different Datetime formats

#### File

- Read contents of a file to a string
- Write a string content into a file

#### Hash

- Calculate SHA256 hash for an arbitrary input
- Calculate SHA256 hash for a file
- Generate a UUID 

#### Path

- Get HOME directory path
- Expand ~ to a full HOME path

#### Shell

- Execute a command or a list of commands in a shell

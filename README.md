<h1>commons</h1>
<h2>Private common library</h2>

------------------------------------------------------------------------

## Content of _commons_

### Traits

#### Collections

`Contains` trait simplifying checking for element's existence inside a collection.

```rust
    pub trait Contains<T: Eq> {
        fn has(&self, item: &T) -> bool;
    }
```

### Utils

#### Date time

- Current date, current timestamp
- Convert between different datetime formats

#### File

- Read contents of a file to a string
- Write a string content into a file

#### Hash

- Calculate _SHA256_ hash for an arbitrary input
- Calculate _SHA256_ hash for a file
- Generate a _UUID_ 

#### Path

- Get _HOME_ directory path
- Expand ~ to a full _HOME_ path

#### Shell

- Execute a command or a list of commands in a shell

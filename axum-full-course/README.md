# A Axum course

From: https://www.youtube.com/watch?v=XZtlD_m59sM


## Tip: Using cargo watch

```bash
# make cargo-watch is installed
cargo install cargo-watch

# run the server code, and reload when application code changes
cargo watch -q -c -w src/ -x run

# run the test code, and reload when tests change
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

## Part 1: Beginner server

- setup the two watchers in two terminals side by side

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

# Tip: cURL with cookies

```bash
# the --cookie-jar flag tells curl where to save response cookies
curl --request POST \
    --header "Content-Type: application/json" \
    --data '{ "username": "demo1", "password": "welcome" }' \
    --cookie-jar cookies.txt \
    localhost:8080/api/login

# the --cookie flag tells curl which cookies to send with the request
curl --request POST \
    --header "Content-Type: application/json" \
    --data '{ "title": "my ticket" }' \
    --cookie cookies.txt \
    localhost:8080/api/tickets
```

## Part 1: Beginner server

- setup the two watchers in two terminals side by side

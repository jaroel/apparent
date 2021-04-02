## Server integration example

Example of a workspace with [Actix](https://actix.rs/) server.

Client:
 - Is served by Actix.
 - Consists of Fetch API examples. Examples are submodules.

---

```bash
cargo make start
```

Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

oha http://localhost:8000/api/kekjo -z 10sec
oha http://127.0.0.1:8000/api/kekjo -z 10sec


Client development:

terminal 1
> cargo install cargo-watch
> cargo make watch_client
terminal 2
> cargo make start_release

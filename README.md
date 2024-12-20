Simple Rust + htmx TODO app using Postgres as persistence layer.

---

`docker compose up` to run this docker-compose (Use `-d` for detached mode and `--build` if you need to [re]build it).

Docker *should* host the docker-compose network at localhost and so you *should* be able access the app here:
http://localhost:3000.

`docker compose down` to gracefully shutdown the app (& db).

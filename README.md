## START DB
`docker-compose up -d`

## RUN MIGRATION
`diesel migration run`

## START SERVER
`cargo run`

## WATCH SERVER
`cargo watch -x run`

## API

### get todo list
```
curl http://127.0.0.1:8080/todos
```

### create todo
```
curl -X POST http://127.0.0.1:8080/todo \
-H "Content-Type: application/json" \
-d '{"content": "Hello todo"}'
```

### update todo
```
curl -X PUT http://127.0.0.1:8080/todo/6 \
-H "Content-Type: application/json" \
-d '{"is_done": true}'
```
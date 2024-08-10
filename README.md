  curl -X POST http://127.0.0.1:8080/todos \
     -H "Content-Type: application/json" \
     -d '{"note": "This is a new todo item"}'


  curl http://127.0.0.1:8080/todos/1

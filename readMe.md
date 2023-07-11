# Connect4Solver_rust
Connect4 game with a minimax 'AI'    
Backend: Rust  
Game mode:
- Human VS Human
- Human VS AI
- AI VS AI

# Front end
https://github.com/RomainMichau/Connect4_UI

# Online version
https://connect4-rust.romainmic.com/

# Also exists in go
https://github.com/RomainMichau/Connect4Solver_go

# Generate OAS

```
swagger generate spec  -w server -o ./server/specs/spec.json   
```

```
swagger generate spec  -w server -o ./server/specs/spec.json   &&  openapi-generator-cli generate -i ./server/specs/spec.json -g typescript-angular -o ./webapp/src/services 
```


# Run with docker
```
 docker build -t connect4 . && docker run -p 8081:8081 connect4:latest
```
http://localhost:8081

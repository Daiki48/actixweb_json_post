# Post json using actix-web.

## Usage

```
$ cargo run
```

Open another terminal.

```
$ curl -X POST -H 'Content-Type: application/json' -d '{ "str": "Hello actix-web", "num": 100, "arr": [1, 2, 3, 4, 5, 6, 7]}' http://localhost:8080/postjson
```

Response

```
str Hello actix-web num 100 arr [1, 2, 3, 4, 5, 6, 7]
```

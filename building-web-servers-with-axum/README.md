# Introduction to Axum

## Hello world

```
make start-hello-world
```

Visit [http://localhost:3000](http://localhost:3000).

## Extractors

```
make start-extractors
```

Issue this curl request:

```
curl -X POST http://localhost:3000 -d "Tacos"
```

## Multiple extractors

```
make start-multiple-extractors
```

Issue this curl request:

```
curl -X POST "http://localhost:3000" -d "Tacos" -H "Authorization: my-secret-key"
```

## Debug handler

```
make start-debug-handler
```

This will fail with an error message explaining the error.

## REST endpoint

```
make start-rest
```

Issue this curl request:

```
curl -X POST "http://localhost:3000" -d '{"name":"Jose"}' -H 'Content-Type: application/json'
```

## Errors

```
make start-errors
```

And try the following requests:

```
curl "http://localhost:3000/no-response" -v
curl "http://localhost:3000/ok" -v
curl "http://localhost:3000/internal-error" -v
curl "http://localhost:3000/bad-request" -v
curl "http://localhost:3000/with-result" -v
curl "http://localhost:3000/custom-client-error" -v
curl "http://localhost:3000/custom-server-error" -v
```

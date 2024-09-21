# Implementing Middleware in Rust

Sometimes you want logic which applies to group of paths or indeed all paths.
* Unlike a handler, middleware is called on every request and path that it's registered on.

## Middleware as a stack
```
        requests
           |
           v
+----- layer_three -----+
| +---- layer_two ----+ |
| | +-- layer_one --+ | |
| | |               | | |    
| | |    handler    | | |
| | |               | | |
| | +-- layer_one --+ | |
| +---- layer_two ----+ |
+----- layer_three -----+
           |
           v
        responses
```
## Applications of middleware
* Authentication
* Logging
* Compression and other response optimizations
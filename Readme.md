# log-server

Simple echo server from [hyper guide echo(https://hyper.rs/guides/server/echo/)

## utf8 supports?

In example code of echo server in hyper's guide, next code: 

```rust
&Method::POST, "/echo/reversed") => {
    let reversed = req.into_body().concat2().map(move |chunk| {
        let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
        *response.body_mut() = Body::from(body);
        response
        });
    return Box::new(reversed);                    
}
```

It makes utf8 string broken.

```rust
(&Method::POST, "/echo/reverse") => {
    *response.body_mut() = Body::wrap_stream(req.into_body().map(|chunk| {
        str::from_utf8(&chunk)
        .unwrap_or("")
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
    }))
}
```

This code works.


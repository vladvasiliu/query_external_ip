# query_external_ip

Get the external IPv4 and IPv6 of the computer.

## Example

```rust
use query_external_ip::Consensus;

async fn get_ip() {
    match Consensus::get().await {
        Ok(c) => println!("{:#?}", c),
        Err(err) => println!("{}", err),
    }
}
```

The sources used provide this service for free, so please don't abuse their kindness.

As the external IP doesn't change all that often, I'd recommended waiting at least 10 minutes between queries.


## Project status

This is an early beta. It only queries IPs from a bunch of HTTP endpoints.

More tests need to be written.

Contributions, in the form of pull requests and issues are welcome.

## Inspiration and similar libraries

This is strongly inspired by [Dario Meloni's](https://github.com/mellon85) [external-ip](https://github.com/mellon85/external-ip).

The main reason for building this new crate was to provide both IPv4 and IPv6 addresses.

There's also [rust-public-ip](https://github.com/avitex/rust-public-ip) which seems to do the same thing.


## License

This program is released under the terms of the [BSD 3-Clause license](LICENSE). You may
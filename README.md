# Ditto Client

A simple [Ditto](https://github.com/phuongtran7/Ditto) client.


## Build

1. Clone [Eclipse Paho MQTT Rust Client](https://github.com/phuongtran7/ditto_client).
2. Change the `path` of Paho MQTT in `Cargo.toml`.
3. Build `ditto_client` with `cargo build`.

## Usage

```shell
PS> .\ditto_client.exe --help
Ditto Client 0.1.0
Receive and parse Ditto output

USAGE:
    ditto_client.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>    Sets the MQTT broker address [default: 127.0.0.1]
    -t, --topic <topic>        Sets the topic to subscribe [default: TestTopic]
```
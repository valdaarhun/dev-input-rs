## About

Reads and decodes raw input events from devices at `/dev/input/event<n>`.

## Usage

```sh
$ cargo build --release
$ cd ./target/release/
$ sudo ./dev-input-rs devpath
```

Running this application requires root privileges.

## References

1. [Input subsystem](https://www.kernel.org/doc/html/latest/input/input.html)
2. [Input event codes](https://www.kernel.org/doc/html/latest/input/event-codes.html)
3. [input-event-codes.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h)
4. [uinput module](https://www.kernel.org/doc/html/latest/input/uinput.html)

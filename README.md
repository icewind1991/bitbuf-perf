# BitBuffer Performance testing

Performance comparision for various rust "bit buffer" implementations.

## Testing method

- As byte source, 1MB of "`0b0000_0001`" bytes are used
- The "bit buffer" is used to read `N` bytes into a `u32`
- This is repeated until the end of the buffer, summing the result of every read into a `u32` (wrapping on overflow)
- This process is repeated 10 times, and the read speed is calculated by averaging all 10 runs

## Results

These results are from a Threadripper 1950X at stock speeds, using rust 1.34.0-nightly in release mode on Linux

| Implementation                   | `N = 1`   | `N = 5`    | `N = 8`    | `N = 10`   | `N = 20`   |
| -------------------------------- | --------  | ---------- | ---------- | ---------- | ---------- |
| [bitreader][bitreader]           | 37.00MB/s | 98.83MB/s  | 119.41MB/s | 127.02MB/s | 150.45MB/s |
| [bitstream_io][bitstream_io]     | 33.18MB/s | 110.69MB/s | 109.76MB/s | 144.01MB/s | 205.20MB/s |
| [bitstream_reader][bitstream_reader] | 69.85MB/s | 349.36MB/s | 558.16MB/s | 670.59MB/s | 1.36GB/s   |

[bitreader]: https://github.com/irauta/bitreader
[bitstream_io]: https://github.com/tuffy/bitstream-io
[bitstream_reader]: https://github.com/icewind1991/bitstream_reader
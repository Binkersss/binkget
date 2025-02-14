## Binkget: A wget clone
Binkget is a wget clone, built using a tokio asynchronous runtime alongside the reqwest https library.
## Installation
If you have cargo installed you can run `cargo install binkget`. Otherwise you can build the binary from source.
## Usage
Use -h flag for help, or -V flag for version.
```
~/$ binkget https://ash-speed.hetzner.com/1GB.bin

URL: https://ash-speed.hetzner.com/1GB.bin
HTTP request sent.. 200 ok
Length: 1073741824 (1.00 GiB)
Type: application/octet-stream
Saving to: 1GB.bin
1GB.bin   [00:01:35] [========] 1.00 GiB/1.00 GiB eta: 0s
Download complete!
```


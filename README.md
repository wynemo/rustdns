# rustdns

it's a simple dns server. written with rust, using tokio.

by default, using `114.114.114.114` as upstream server.

usage:

`./updns [local_addr] [remote_addr]`

for example:

`./updns [127.0.0.1:8080] [114.114.114.114:53]`


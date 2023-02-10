# Rust crates for gRPC interfaces

This is composed of 3 crates:

1. `mc-probufs-tonic`: A crate providing a
   [Tonic](https://docs.rs/tonic/latest/tonic/) implementation of gRPC client
   and server services.
2. `mc-probufs-grpcio`: A crate providing a
   [grpcio](https://docs.rs/grpcio/latest/grpcio/) implementation of gRPC client
   and server services.
3. `mc-probufs-messages`: A crate providing gRPC messages. This is used by
   both of the service implementations, `mc-probufs-tonic` and `mc-probufs-grpcio`.

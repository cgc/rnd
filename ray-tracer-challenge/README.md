# "The Ray Tracer Challenge" in Rust

This repo contains my code to implement the ray tracer described in The Ray Tracer Challenge by Jamis Buck.

Run tests with
```
cargo test
```

Generate some images (into `output`) with
```
cargo run --release
```

To profile, I used `flamegraph`:
```
cargo install flamegraph
cargo run --release
sudo flamegraph -- ./target/release/ray_tracer_challenge
open flamegraph.svg
```

Tests were generated from the `*.feature` tests from the book, by executing `python tests/testgen.py`.

## Tests from book

Files in `book-code` were downloaded from the book's code from the [forum](https://forum.devtalk.com/t/the-ray-tracing-challenge-including-the-tests-from-source-code-file-in-an-open-source-repository/29081). The `forum-scenes` folder contains `*.yml` scene descriptions that were posted by Jamis in the [forum](https://forum.raytracerchallenge.com/board/4/gallery).

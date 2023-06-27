# HttpGet

[![release](https://github.com/taliamax/httpget/actions/workflows/release.yaml/badge.svg)](https://github.com/taliamax/httpget/actions/workflows/release.yaml)
[![test](https://github.com/taliamax/httpget/actions/workflows/test.yaml/badge.svg)](https://github.com/taliamax/httpget/actions/workflows/test.yaml)

This is a tiny client that sends an HTTP GET request to the parameter that you specify on the command line. This is statically compiled, so it is useful to toss into distroless or scratch containers that need to have an HTTP client for health check purposes.

## Why build this?

This project came to be because a friend wanted to run health checks in a distroless container, and I wanted to make a binary that was as small as it could be. I did a writeup about that process, so if you want more information please read [this blog post](https://natalia.dev/blog/2023/03/docker-health-checks-on-distroless-containers-with-rust/).

It's also becoming a bigger trend to use either [distroless](https://github.com/GoogleContainerTools/distroless#why-should-i-use-distroless-images) containers, or containers based on [`scratch`](https://support.snyk.io/hc/en-us/articles/360004012857-What-are-docker-scratch-based-images-) for security/image size reasons; this project allows a minimal dependency client that can only do GET requests, and has negligible footprint on container size.

### Just how small is this?

Let's compare the binary size against other popular ones, specifically against `wget` and `curl`, measured from the binary & its dependencies in the `alpine` container

| Binary                   | Size   |
| ------------------------ | ------ |
| `curl`                   | 6.1mb  |
| `wget`                   | 1.4mb  |
| `httpget`, no TLS        | 610kb  |
| `httpget`, with `rustls` | 1.62mb |

So, all in all, it's quite minimal.

## Installing HttpGet

Currently, only x86_64 binaries are released. If you are hoping to use this with ARM-based images, you will need to build them yourself.

### Docker

Since this binary is primarily meant to be used for Docker health checks, the easiest way to consume this binary is through Docker. The binaries are published in the `ghcr.io/taliamax/httpget` repository in scratch containers, and you can use the version tags or `latest`/`latest-tls`.

Any released version tag includes an equivalent `tls` tag. See below for an example Dockerfile, or see [this project](https://github.com/taliamax/httpget-example/blob/main/Dockerfile) for an example usecase.

```dockerfile
FROM golang:latest as build

# Build steps here
# (...)

# In the end we get a binary at /app/bin/example with our web service

FROM ghcr.io/taliamax/httpget:latest as httpget

FROM scratch as runner

COPY --from=build /app/bin/example /example
COPY --from=httpget /httpget /httpget

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD ["/httpget", "http://localhost:8080/healthz"]

CMD ["/example"]
```

Another option is to put the `httpget` image _before_ the build stage and copy it into the output directory for the build stage, so your final image is only a single layer:

```dockerfile
FROM ghcr.io/taliamax/httpget:latest as httpget

FROM golang:latest as build

# Build steps here
# (...)

# In the end we get a binary at /app/bin/example with our web service

COPY --from=httpget /httpget /app/bin/httpget

FROM scratch as runner

COPY --from=build /app/bin /bin

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD ["/bin/httpget", "http://localhost:8080/healthz"]

CMD ["/bin/example"]
```

### Github Releases (Direct Download)

The regular binary and the TLS binary are available through Github Releases, and can be downloaded [here](https://github.com/taliamax/httpget/releases). The `httpget` binary can _only_ use `http://` protocol URLs, and the `httpget-tls` can use either `http://` or `https://`.

### Install with Cargo

This project is published on [crates.io](https://crates.io/crates/httpget), and can be installed using `cargo` with the command `cargo install httpget`. This will **not** produce a statically-linked binary: for that, you must ensure that you've installed the correct `*-unknown-linux-musl` target.

You can also clone this repository and run `cargo install --path .` to install through Cargo

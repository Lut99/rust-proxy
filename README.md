# rust-proxy: A simple, containerized proxy service with TLS support written in Rust
The `rust-proxy` project aims to implement a simple TLS-enabled proxy for any project, or otherwise multiple projects. Using an intuitive, minimal TOML-file to set it up, the idea is to be as quickly as possible in setting up all your proxying needs.


## Syntax
The selling point of this repository is its syntax, and so we will completely introduce it here in hopefully a small enough space to convince you.

### Config
Every configuration is always supposed to have a `[config]`-block. This will contain general configuration for the proxy server.

Currently, it supports the following fields:
- `tls`: Determines how to use TLS. Can either be `"none"`, `"optional"` or `"mandatory"`. Omitting this field is equivalent to setting it to `"none"`.
- `nonmatched`: Determines what to do with URLs that are not matched to a rule at all. They can be `"dropped"` or `"allowed"`. The first case generates a 

If the `tls` option is not `none`, then a configuration file must be provided during image build (see [below](#installation)).

### Defining mappings
To define URL/port mappings, you can use the `[mappings]`-block to specify any number of them. Each field in this block is a key/value pair of the url (as a string) mapped to the URL or port to map to.

The URL on the lefthand-side can be fairly complex to match any URL. In fact, it supports full regular expression syntax (as defined in [todo](https://todo.com)) to defined the matched URLs.

Similarly, the righthand-side can be used to 'generate' a new URL. While this is not full regex, it does use the same syntax as VS Code to use matched blocks; any (non-nested) parenthesis in the key can be accessed by using `$x` in the value, where `x` is the index of the group in the regex.

For example:
```toml
[mappings]
# Redirects this specific URL to port 80
"http://heeeeeeeey\.com/" = 80

# Redirects this specific URL to port 443
"https://heeeeeeeey\.com/" = 443

# Redirects any `http` URL to a specific website
# (Note the lack of `\` before the dot)
"https://.*" = "https://heeeeeeeey.com/"

# Forces any request to be connected through TLS
"http://(.*)" = "https://$1"

# Changes the protocol for websites with at least a nested path
"https://([^/]*)/(.*)" = "file://$1/$2"
```

Finally, note that the proxy service may actually match multiple rules if they all match. In that case, all matching ones are applied in-order:

```toml
[mappings]
# We can build rules; first, we always connect with https...
"http://(.*)" = "https://$1"

# ...and then we can proxy individual addresses:
"https://test-1\.nl.*" = 443
"https://test-2\.nl.*" = 444
```

Note that this may result in addresses that are recursive. Fortunately, the installer will actually check if this is the case (see [below](#installation)).



## Installation
To install the project, clone the repository first.

Then, you should install [Rust](https://rustup.rs) and [Docker](https://docker.com). You should also install Docker's [BuildKit plugin](https://github.com/docker/buildx) if you intend to use the installer in this repository (check the [rust-build](https://github.com/Lut99/rust-build) project!).

Write a configuration for your proxy before proceeding; it should be in the root of the repository called `config.toml` (check the [syntax](#config-syntax) section).

With the dependencies installed and the config prepared, navigate to the repository's root directory and run:
```bash
cargo build --release --package proxy-setup
./target/release/proxy-setup install
```

This will create and then load an `image.tar` file with the proxy binary and the configuration. Alternatively, run
```bash
cargo build --release --package proxy-setup
./target/release/proxy-setup build
```
to just create the `image.tar` file to move it to another machine to run there. Note that Docker and its plugin are still required to be installed to build the image.

### TLS
If you want to use TLS, you have to provide the certificate to use. To do so, run the installer with the `cert`-flag:
```bash
./target/release/proxy-setup install --cert <path_to_certificate>
```

### Other file
To use another configuration file than the default `config.toml`, add the `file`-flag:
```bash
./target/release/proxy-setup install --file <path_to_file>
```

### Checks
The installer performs a few checks on the `config.toml` file before installing (e.g., whether any rules cause infinite recursion). To prevent it from doing so for whatever reason, add the `skip-checks`-flag:
```bash
./target/release/proxy-setup install --skip-checks
```



## Running

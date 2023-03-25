# Flow

[![Clippy](https://github.com/scattered-systems/flow/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/clippy.yml)
[![Docker](https://github.com/scattered-systems/flow/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/flow/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/rust.yml)

***

Flow encompasses a host of capabilities native to the ecosystem.

## Installation

Make sure you have docker installed on the target system

### *Pull the image*

```bash
docker pull scsys/flow:latest
```

### *Build the image locally (optional)*

```bash
docker buildx build --tag scsys/flow:latest .
```

### *Run the image*

```bash
docker run -p 9090:9090 scsys/flow:latest
```

## Usage

```bash

```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)

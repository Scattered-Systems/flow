# Flow

[![Clippy](https://github.com/scattered-systems/flow/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/clippy.yml)
[![Docker](https://github.com/scattered-systems/flow/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/flow/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/flow/actions/workflows/rust.yml)

***

Flow describes the core node logic that all network participants must run. With the extensive integrations with Proton, Flow essentially becomes a 
type of headless operating system capable of synchronizing its activites across devices maximizing the users control and available resources.

In order to accomplish this, Flow implements a novel harmonic orchestration mechanism used to describe the behaviour of the networks and the execution 
of individual transactions. [Contained](https://github.com/FL03/contained) is derivied from the Neo-Riemannian theory, elegantly blending
harmonics and music theory with distributed / multi-way systems. 

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

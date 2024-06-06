
# ipfs-one

A simple aggregator for IPFS gateways.

## Usage

Clone the repository:

```bash
git clone https://github.com/project-novi/ipfs-one.git
cd ipfs-one
```

Create a config file named `config.yaml`. Example config:

```yaml
bind_address: '127.0.0.1:3345'
gateways:
  - http://127.0.0.1:8080
```

Run the aggregator:

```bash
RUST_LOG=info cargo run --release
```

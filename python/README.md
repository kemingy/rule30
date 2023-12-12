# Rule30

## Installation

```bash
pip install rule30py
```

## Usage

```python
from rule30 import random, Rule30Random

print(random())

# to use the Psuedo-Random Number Generator
rng = Rule30Random()
print(rng.random())
print(rng.getrandbits(8))
```

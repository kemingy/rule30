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
# all the methods of random.Random are available, such as:
print(rng.randint(0, 100))
print(rng.uniform(0, 1))
print(rng.choice(range(10))
print(rng.sample(range(10), 3))
```

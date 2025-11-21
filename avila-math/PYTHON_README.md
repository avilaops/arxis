# avila-math Python Bindings

Python interface for the avila-math Rust library.

## Installation

```bash
# Install maturin
pip install maturin

# Build and install
cd avila-math
maturin develop --features python
```

## Usage

### Quaternions

```python
import avila_math

# Create quaternions
q1 = avila_math.Quaternion.identity()
q2 = avila_math.Quaternion.from_axis_angle([0, 1, 0], 1.5708)  # 90° around Y

# Multiply
q3 = q1 * q2

# SLERP interpolation
q_mid = q1.slerp(q2, 0.5)

# Rotate a vector
v = [1.0, 0.0, 0.0]
rotated = q2.rotate_vector(v)
print(f"Rotated: {rotated}")  # [0, 0, -1] (approximately)
```

### Tensors (NumPy Integration)

```python
import avila_math
import numpy as np

# Create tensor from shape
tensor = avila_math.Tensor([2, 3, 4], 0.0)
tensor.fill(1.5)

print(f"Shape: {tensor.shape()}")
print(f"Size: {tensor.size()}")

# Convert to NumPy
arr = tensor.to_numpy()
print(arr.shape)  # (2, 3, 4)

# Create from NumPy
arr2 = np.random.randn(5, 5)
tensor2 = avila_math.Tensor.from_numpy(arr2)

# Operations
tensor3 = tensor2.scale(2.0)
print(f"Mean: {tensor3.mean()}")
```

### Autograd

```python
import avila_math

# Create tape for automatic differentiation
tape = avila_math.Tape()

# Create variables
x = tape.var(2.0)
y = tape.var(3.0)

# Build computation graph
z = (x * y) + x  # z = 2*3 + 2 = 8

# Backward pass
tape.backward(z)

# Get gradients
print(f"dz/dx = {tape.grad(x)}")  # 4.0 (y + 1)
print(f"dz/dy = {tape.grad(y)}")  # 2.0 (x)
```

### Advanced Operations

```python
# Power
a = tape.var(2.0)
b = a ** 3  # a^3 = 8
tape.backward(b)
print(f"d(a^3)/da = {tape.grad(a)}")  # 12.0 (3*a^2)

# Exponential and log
tape2 = avila_math.Tape()
x = tape2.var(1.0)
y = x.exp()  # e^1
z = y.log()  # ln(e^1) = 1
tape2.backward(z)
print(f"Value: {z.value()}")  # 1.0
```

## Examples

Run the Rust examples to see the library in action:

```bash
# Quaternion animation
cargo run --example quaternion_animation --release

# Kalman filter tracking
cargo run --example kalman_tracking --release

# Wiener filter denoising
cargo run --example wiener_denoising --release

# Autograd neural network
cargo run --example autograd_xor --release
```

## Features

- **Quaternions**: Full quaternion algebra with SLERP
- **Tensors**: N-dimensional arrays with NumPy interop
- **Autograd**: Tape-based automatic differentiation
- **Pure Rust**: Zero Python/C dependencies in computation

## Performance

All heavy computation is done in Rust with SIMD optimizations. Python overhead is minimal for large operations.

## License

MIT OR Apache-2.0

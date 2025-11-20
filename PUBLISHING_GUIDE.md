# 📦 Publishing Guide - Avila Rust Ecosystem

Complete guide for publishing Avila crates to crates.io and beyond.

---

## 🎯 **Publishing Checklist**

### **Pre-Publishing (Complete before `cargo publish`)**

- [x] ✅ **LICENSE files** created (MIT + Apache-2.0)
- [x] ✅ **CHANGELOG.md** with version history
- [ ] ⚠️ **Cargo.toml metadata** complete
- [ ] ⚠️ **README.md** for each crate
- [ ] ⚠️ **API documentation** (/// comments)
- [ ] ⚠️ **Tests passing** (`cargo test --all`)
- [ ] ⚠️ **Examples documented**
- [ ] ⚠️ **GitHub repository** public
- [ ] ⚠️ **Crates.io account** setup

---

## 📝 **Step 1: Complete Cargo.toml Metadata**

### **Required fields for crates.io:**

```toml
[package]
name = "avila-math"
version = "0.1.0"
edition = "2021"
authors = ["Nicolas Ávila <nicolas@avila.inc>"]
license = "MIT OR Apache-2.0"
description = "Mathematical kernel for Avila: quaternions, tensors, 4D geometry"
repository = "https://github.com/avilaops/arxis"
homepage = "https://avila.cloud"                # ADD THIS
documentation = "https://docs.rs/avila-math"    # ADD THIS
readme = "README.md"                             # ADD THIS
keywords = ["math", "quaternions", "tensors", "4d", "geometry"]  # Max 5
categories = ["mathematics", "science", "algorithms"]            # Max 5
exclude = [                                      # ADD THIS
    "target/",
    "*.swp",
    ".git*",
    "examples/*.ppm",
    "output_*.ppm"
]
```

### **Update ALL Cargo.toml files:**

1. ✅ `avila-math/Cargo.toml`
2. ✅ `avila-telemetry/Cargo.toml`
3. ⚠️ `avx-quantum-render/Cargo.toml`
4. ⚠️ `avx-config/Cargo.toml`
5. ⚠️ `avx-telemetry/Cargo.toml`
6. ⚠️ `avx-gateway/Cargo.toml`
7. ⚠️ `avx-api-core/Cargo.toml`
8. ⚠️ `avx-cli/Cargo.toml`
9. ⚠️ `avx-events/Cargo.toml`
10. ⚠️ `avx-image/Cargo.toml`
11. ✅ `arxis_quaternions` (root Cargo.toml)

---

## 📚 **Step 2: Add API Documentation**

### **Add rustdoc comments to public APIs:**

```rust
/// A 3D quaternion for representing rotations in 3D space.
///
/// Quaternions provide a compact, singularity-free representation
/// of rotations, avoiding gimbal lock issues present in Euler angles.
///
/// # Examples
///
/// ```
/// use avila_math::geometry::Quat3D;
/// use std::f64::consts::PI;
///
/// // Create rotation around Y-axis by 90°
/// let q = Quat3D::from_axis_angle([0.0, 1.0, 0.0], PI / 2.0);
///
/// // Rotate a vector
/// let v = [1.0, 0.0, 0.0];
/// let rotated = q.rotate_vector(v);
/// ```
///
/// # Mathematical Foundation
///
/// A quaternion is represented as q = w + xi + yj + zk where:
/// - w is the scalar (real) part
/// - (x, y, z) is the vector (imaginary) part
/// - i² = j² = k² = ijk = -1
pub struct Quat3D {
    /// Scalar (real) component
    pub w: f64,
    /// Vector i component
    pub x: f64,
    /// Vector j component
    pub y: f64,
    /// Vector k component
    pub z: f64,
}
```

### **Document ALL:**
- Public structs
- Public functions
- Public methods
- Module-level docs
- Crate-level docs (in lib.rs)

---

## 🧪 **Step 3: Ensure Tests Pass**

```powershell
# Test all workspace members
cd "c:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\1.2.7 - Identidade visual\Arxis"
cargo test --all --release

# Test specific crate
cargo test -p avila-math --release

# Run examples
cargo run --example geometry4d_example --release
cargo run --example conv4d_example --release

# Check documentation builds
cargo doc --all --no-deps
```

---

## 🔐 **Step 4: Setup Crates.io Account**

1. **Create account**: https://crates.io/
2. **Get API token**: https://crates.io/me
3. **Login via cargo**:
   ```powershell
   cargo login <your-token>
   ```
4. **Token is saved** in `~/.cargo/credentials`

---

## 🚀 **Step 5: Publish (Ordem de Dependências)**

### **CRITICAL: Publish in dependency order!**

```powershell
cd "c:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\1.2.7 - Identidade visual\Arxis"

# 1. Base crates (no internal dependencies)
cargo publish -p avila-math
Start-Sleep -Seconds 30  # Wait for crates.io to index

cargo publish -p avila-telemetry
Start-Sleep -Seconds 30

cargo publish -p avx-config
Start-Sleep -Seconds 30

# 2. Mid-level crates (depend on base)
cargo publish -p avx-telemetry
Start-Sleep -Seconds 30

cargo publish -p avx-quantum-render
Start-Sleep -Seconds 30

cargo publish -p avx-image
Start-Sleep -Seconds 30

# 3. Application crates
cargo publish -p avx-gateway
Start-Sleep -Seconds 30

cargo publish -p avx-api-core
Start-Sleep -Seconds 30

cargo publish -p avx-cli
Start-Sleep -Seconds 30

cargo publish -p avx-events
Start-Sleep -Seconds 30

# 4. Main crate (depends on everything)
cargo publish -p arxis_quaternions
```

### **Important Notes:**
- ⏰ Wait 30-60s between publishes for crates.io indexing
- 🔄 Cannot unpublish (only yank versions)
- 📦 Max upload size: 10 MB (use `exclude` in Cargo.toml)
- 🔢 Version must be unique (can't republish same version)

---

## 📊 **Step 6: Post-Publishing**

### **Verify on crates.io:**
```
https://crates.io/crates/avila-math
https://crates.io/crates/avila-telemetry
https://crates.io/crates/arxis_quaternions
```

### **Check docs.rs builds:**
```
https://docs.rs/avila-math
https://docs.rs/arxis_quaternions
```

### **Add badges to README.md:**
```markdown
[![Crates.io](https://img.shields.io/crates/v/avila-math.svg)](https://crates.io/crates/avila-math)
[![Documentation](https://docs.rs/avila-math/badge.svg)](https://docs.rs/avila-math)
[![License](https://img.shields.io/crates/l/avila-math.svg)](https://github.com/avilaops/arxis#license)
[![Downloads](https://img.shields.io/crates/d/avila-math.svg)](https://crates.io/crates/avila-math)
```

---

## 🌐 **Step 7: Promote Across Platforms**

### **1. GitHub**
- [x] Repository: https://github.com/avilaops/arxis
- [ ] Add Topics: `rust`, `mathematics`, `physics`, `4d-geometry`, `gravitational-waves`
- [ ] Enable GitHub Discussions
- [ ] Create Release with CHANGELOG

### **2. Reddit Announcement**
```
Title: [Announcement] Avila - Rust ecosystem for 4D geometry, tensor ops, and physics

r/rust - Main announcement
r/Physics - Focus on LISA mission integration
r/GraphicsProgramming - Quantum renderer
r/MachineLearning - Conv4D and tensor operations
```

### **3. Submit to Awesome Lists**
- **Awesome Rust**: https://github.com/rust-unofficial/awesome-rust
  - Categories: Mathematics, Astronomy, Graphics
- **Are We Game Yet**: https://arewegameyet.rs/
- **Are We Learning Yet**: https://www.arewelearningyet.com/

### **4. This Week in Rust**
- Submit: https://github.com/rust-lang/this-week-in-rust
- Format:
  ```
  # New Crates

  * [avila-math](https://crates.io/crates/avila-math) - Mathematical kernel with quaternions, tensors, 4D geometry
  * [arxis_quaternions](https://crates.io/crates/arxis_quaternions) - Physics engine for gravitational waves and relativity
  ```

### **5. Social Media**
- Twitter/X: @avilaops (if exists)
- LinkedIn: Nicolas Ávila profile
- Mastodon: rust community instances

---

## 🔄 **Step 8: Version Updates**

### **Semantic Versioning:**
- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes

### **Update version:**
```powershell
# Update version in Cargo.toml
# Update CHANGELOG.md

# Commit changes
git commit -am "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags

# Publish new version
cargo publish -p avila-math
```

---

## 📈 **Metrics & Monitoring**

### **Track on:**
1. **Crates.io stats**: Downloads, recent downloads
2. **Docs.rs**: Build status, failed builds
3. **GitHub**: Stars, forks, issues, PRs
4. **Lib.rs**: Alternative view with feature analysis

### **Goals:**
- 📦 **Week 1**: 100+ downloads
- 🌟 **Month 1**: 50+ GitHub stars
- 📚 **Month 3**: Featured in Awesome Rust
- 🚀 **Month 6**: 1,000+ downloads

---

## 🆘 **Troubleshooting**

### **Common Issues:**

1. **"crate not found" during publish**
   - Wait 1-2 minutes for crates.io indexing
   - Check you published dependencies first

2. **"too large" error**
   - Add `exclude` in Cargo.toml
   - Remove `target/`, large test files

3. **Documentation fails to build**
   - Test locally: `cargo doc --no-deps`
   - Check for missing doc dependencies

4. **Version already published**
   - Bump version number
   - Cannot re-use version numbers

---

## 📞 **Support**

- **Email**: nicolas@avila.inc
- **GitHub Issues**: https://github.com/avilaops/arxis/issues
- **WhatsApp**: +55 17 99781-1471

---

## ✅ **Final Checklist Before Publishing**

- [ ] All tests pass (`cargo test --all`)
- [ ] Documentation builds (`cargo doc --all`)
- [ ] Examples run successfully
- [ ] LICENSE files in root
- [ ] CHANGELOG.md updated
- [ ] Cargo.toml metadata complete
- [ ] README.md for each crate
- [ ] GitHub repository public
- [ ] Crates.io account setup
- [ ] API token configured
- [ ] Dependencies published first
- [ ] Version numbers unique

**Ready to publish? Run:**
```powershell
.\scripts\publish_all.ps1
```

---

**Good luck! 🚀**

//! Native feature detection
//!
//! Pure Rust implementations of:
//! - Harris corner detector
//! - FAST (Features from Accelerated Segment Test)
//! - ORB (Oriented FAST and Rotated BRIEF)
//! - Feature matching

use crate::native::buffer::NativeImageBuffer;
use crate::native::convolution::convolve_2d;
use crate::native::math::fast_atan2;

/// Keypoint detected in an image
#[derive(Debug, Clone)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub response: f32,
    pub size: f32,
    pub angle: f32,
    pub octave: u8,
}

impl Keypoint {
    pub fn new(x: f32, y: f32, response: f32) -> Self {
        Self {
            x,
            y,
            response,
            size: 1.0,
            angle: -1.0,
            octave: 0,
        }
    }
}

/// Feature descriptor (binary or float vector)
#[derive(Debug, Clone)]
pub struct Descriptor {
    pub data: Vec<u8>, // Binary descriptor (ORB, BRIEF)
}

/// Feature match between two keypoints
#[derive(Debug, Clone)]
pub struct Match {
    pub query_idx: usize,
    pub train_idx: usize,
    pub distance: f32,
}

/// Harris corner detector
///
/// Detects corners using the Harris corner response function:
/// R = det(M) - k * trace(M)^2
/// where M is the structure tensor
pub fn harris_corners(
    img: &NativeImageBuffer,
    threshold: f32,
    k: f32,
) -> std::io::Result<Vec<Keypoint>> {
    if img.channels != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Harris detector requires grayscale image",
        ));
    }

    let width = img.width as usize;
    let height = img.height as usize;

    // Sobel kernels for gradient computation
    let sobel_x = vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let sobel_y = vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

    // Compute gradients
    let ix = convolve_2d(&img.data, width, height, &sobel_x, 3);
    let iy = convolve_2d(&img.data, width, height, &sobel_y, 3);

    // Compute structure tensor components
    let mut ixx = vec![0.0; width * height];
    let mut iyy = vec![0.0; width * height];
    let mut ixy = vec![0.0; width * height];

    for i in 0..width * height {
        ixx[i] = ix[i] * ix[i];
        iyy[i] = iy[i] * iy[i];
        ixy[i] = ix[i] * iy[i];
    }

    // Gaussian smoothing of structure tensor (simplified box filter)
    let window = 5;
    let window_size = window * window;
    let mut response = vec![0.0; width * height];

    for y in window / 2..height - window / 2 {
        for x in window / 2..width - window / 2 {
            let mut sxx = 0.0;
            let mut syy = 0.0;
            let mut sxy = 0.0;

            // Sum over window
            for wy in 0..window {
                for wx in 0..window {
                    let idx = (y + wy - window / 2) * width + (x + wx - window / 2);
                    sxx += ixx[idx];
                    syy += iyy[idx];
                    sxy += ixy[idx];
                }
            }

            sxx /= window_size as f32;
            syy /= window_size as f32;
            sxy /= window_size as f32;

            // Harris response: det(M) - k * trace(M)^2
            let det = sxx * syy - sxy * sxy;
            let trace = sxx + syy;
            let r = det - k * trace * trace;

            response[y * width + x] = r;
        }
    }

    // Non-maximum suppression and thresholding
    let mut keypoints = Vec::new();
    let nms_radius = 3;

    for y in nms_radius..height - nms_radius {
        for x in nms_radius..width - nms_radius {
            let idx = y * width + x;
            let r = response[idx];

            if r < threshold {
                continue;
            }

            // Check if local maximum
            let mut is_maximum = true;
            for dy in -(nms_radius as i32)..=(nms_radius as i32) {
                for dx in -(nms_radius as i32)..=(nms_radius as i32) {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    let neighbor_idx = ny * width + nx;

                    if response[neighbor_idx] > r {
                        is_maximum = false;
                        break;
                    }
                }
                if !is_maximum {
                    break;
                }
            }

            if is_maximum {
                keypoints.push(Keypoint::new(x as f32, y as f32, r));
            }
        }
    }

    Ok(keypoints)
}

/// FAST (Features from Accelerated Segment Test) detector
///
/// Detects corners by comparing pixel intensities in a circle around the candidate point
pub fn fast_detector(
    img: &NativeImageBuffer,
    threshold: u8,
    n_consecutive: usize,
) -> std::io::Result<Vec<Keypoint>> {
    if img.channels != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "FAST detector requires grayscale image",
        ));
    }

    let width = img.width as usize;
    let height = img.height as usize;

    // Bresenham circle offsets (radius 3)
    let circle_offsets: [(i32, i32); 16] = [
        (0, 3),
        (1, 3),
        (2, 2),
        (3, 1),
        (3, 0),
        (3, -1),
        (2, -2),
        (1, -3),
        (0, -3),
        (-1, -3),
        (-2, -2),
        (-3, -1),
        (-3, 0),
        (-3, 1),
        (-2, 2),
        (-1, 3),
    ];

    let mut keypoints = Vec::new();
    let margin = 3;

    for y in margin..height - margin {
        for x in margin..width - margin {
            let center_idx = y * width + x;
            let center_val = (img.data[center_idx] * 255.0) as u8;

            // Quick rejection test: check pixels 1, 5, 9, 13
            let test_pixels = [0, 4, 8, 12];
            let mut bright = 0;
            let mut dark = 0;

            for &i in &test_pixels {
                let (dx, dy) = circle_offsets[i];
                let px = (x as i32 + dx) as usize;
                let py = (y as i32 + dy) as usize;
                let idx = py * width + px;
                let val = (img.data[idx] * 255.0) as u8;

                if val as i32 > center_val as i32 + threshold as i32 {
                    bright += 1;
                } else if (val as i32) < center_val as i32 - threshold as i32 {
                    dark += 1;
                }
            }

            if bright < 3 && dark < 3 {
                continue;
            }

            // Full circle test
            let mut consecutive_bright = 0;
            let mut consecutive_dark = 0;
            let mut max_consecutive_bright = 0;
            let mut max_consecutive_dark = 0;

            // Check twice to handle wrap-around
            for i in 0..32 {
                let idx_mod = i % 16;
                let (dx, dy) = circle_offsets[idx_mod];
                let px = (x as i32 + dx) as usize;
                let py = (y as i32 + dy) as usize;
                let idx = py * width + px;
                let val = (img.data[idx] * 255.0) as u8;

                if val as i32 > center_val as i32 + threshold as i32 {
                    consecutive_bright += 1;
                    consecutive_dark = 0;
                    max_consecutive_bright = max_consecutive_bright.max(consecutive_bright);
                } else if (val as i32) < center_val as i32 - threshold as i32 {
                    consecutive_dark += 1;
                    consecutive_bright = 0;
                    max_consecutive_dark = max_consecutive_dark.max(consecutive_dark);
                } else {
                    consecutive_bright = 0;
                    consecutive_dark = 0;
                }
            }

            if max_consecutive_bright >= n_consecutive || max_consecutive_dark >= n_consecutive {
                // Compute response as sum of absolute differences
                let mut response = 0.0;
                for &(dx, dy) in &circle_offsets {
                    let px = (x as i32 + dx) as usize;
                    let py = (y as i32 + dy) as usize;
                    let idx = py * width + px;
                    let val = (img.data[idx] * 255.0) as u8;
                    response += (val as i32 - center_val as i32).abs() as f32;
                }

                keypoints.push(Keypoint::new(x as f32, y as f32, response));
            }
        }
    }

    Ok(keypoints)
}

/// Compute orientation for keypoint (for rotation invariance)
pub fn compute_orientation(img: &NativeImageBuffer, kp: &Keypoint) -> f32 {
    let width = img.width as usize;
    let height = img.height as usize;
    let x = kp.x as usize;
    let y = kp.y as usize;

    let radius = 15;
    if x < radius || y < radius || x >= width - radius || y >= height - radius {
        return 0.0;
    }

    // Compute image moments
    let mut m01 = 0.0;
    let mut m10 = 0.0;

    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let dist_sq = dx * dx + dy * dy;
            if dist_sq > (radius as i32 * radius as i32) {
                continue;
            }

            let px = (x as i32 + dx) as usize;
            let py = (y as i32 + dy) as usize;
            let idx = py * width + px;
            let intensity = img.data[idx];

            m10 += dx as f32 * intensity;
            m01 += dy as f32 * intensity;
        }
    }

    fast_atan2(m01, m10)
}

/// Extract BRIEF descriptor (Binary Robust Independent Elementary Features)
/// Simplified version using random sampling pattern
pub fn extract_brief_descriptor(
    img: &NativeImageBuffer,
    kp: &Keypoint,
    descriptor_size: usize,
) -> Descriptor {
    let width = img.width as usize;
    let height = img.height as usize;
    let x = kp.x as usize;
    let y = kp.y as usize;

    let patch_size = 31;
    let half_patch = patch_size / 2;

    // Binary descriptor (1 bit per comparison)
    let num_bytes = (descriptor_size + 7) / 8;
    let mut data = vec![0u8; num_bytes];

    if x < half_patch || y < half_patch || x >= width - half_patch || y >= height - half_patch {
        return Descriptor { data };
    }

    // Simplified sampling pattern (deterministic pseudo-random)
    let mut rng_state = 42u32;
    let patch_size_u32 = patch_size as u32;
    for bit_idx in 0..descriptor_size {
        // Simple LCG for deterministic "random" sampling
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let x1 = ((rng_state >> 16) % patch_size_u32) as i32 - half_patch as i32;
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let y1 = ((rng_state >> 16) % patch_size_u32) as i32 - half_patch as i32;
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let x2 = ((rng_state >> 16) % patch_size_u32) as i32 - half_patch as i32;
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let y2 = ((rng_state >> 16) % patch_size_u32) as i32 - half_patch as i32;

        let p1_idx = ((y as i32 + y1) as usize) * width + (x as i32 + x1) as usize;
        let p2_idx = ((y as i32 + y2) as usize) * width + (x as i32 + x2) as usize;

        let p1 = img.data[p1_idx];
        let p2 = img.data[p2_idx];

        if p1 > p2 {
            let byte_idx = bit_idx / 8;
            let bit_pos = bit_idx % 8;
            data[byte_idx] |= 1 << bit_pos;
        }
    }

    Descriptor { data }
}

/// ORB: Oriented FAST and Rotated BRIEF
pub fn orb_detect_and_compute(
    img: &NativeImageBuffer,
    max_features: usize,
) -> std::io::Result<(Vec<Keypoint>, Vec<Descriptor>)> {
    // Detect FAST keypoints
    let mut keypoints = fast_detector(img, 20, 9)?;

    // Sort by response and keep top N
    keypoints.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap());
    keypoints.truncate(max_features);

    // Compute orientations
    for kp in &mut keypoints {
        kp.angle = compute_orientation(img, kp);
    }

    // Extract BRIEF descriptors (256 bits)
    let descriptors: Vec<Descriptor> = keypoints
        .iter()
        .map(|kp| extract_brief_descriptor(img, kp, 256))
        .collect();

    Ok((keypoints, descriptors))
}

/// Hamming distance between two binary descriptors
pub fn hamming_distance(a: &Descriptor, b: &Descriptor) -> u32 {
    let mut distance = 0u32;
    for i in 0..a.data.len().min(b.data.len()) {
        let xor = a.data[i] ^ b.data[i];
        distance += xor.count_ones();
    }
    distance
}

/// Match features using brute-force with Hamming distance
pub fn match_features(
    descriptors1: &[Descriptor],
    descriptors2: &[Descriptor],
    max_distance: u32,
) -> Vec<Match> {
    let mut matches = Vec::new();

    for (i, desc1) in descriptors1.iter().enumerate() {
        let mut best_distance = u32::MAX;
        let mut best_idx = 0;

        for (j, desc2) in descriptors2.iter().enumerate() {
            let dist = hamming_distance(desc1, desc2);
            if dist < best_distance {
                best_distance = dist;
                best_idx = j;
            }
        }

        if best_distance <= max_distance {
            matches.push(Match {
                query_idx: i,
                train_idx: best_idx,
                distance: best_distance as f32,
            });
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harris_corners() {
        let mut img = NativeImageBuffer::new(100, 100, 1);

        // Create a corner pattern
        for y in 40..60 {
            for x in 40..60 {
                let idx = y * 100 + x;
                img.data[idx] = if x < 50 && y < 50 { 1.0 } else { 0.0 };
            }
        }

        let corners = harris_corners(&img, 0.01, 0.04).unwrap();
        assert!(corners.len() > 0, "Should detect corners");
    }

    #[test]
    fn test_fast_detector() {
        let mut img = NativeImageBuffer::new(100, 100, 1);

        // Create simple corner features - single bright pixels on dark background
        for y in 0..100 {
            for x in 0..100 {
                let idx = y * 100 + x;
                img.data[idx] = 0.0; // Dark background
            }
        }

        // Add some bright corner points
        for &(x, y) in &[(20, 20), (50, 50), (80, 80), (30, 70), (70, 30)] {
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let px = (x as i32 + dx) as usize;
                    let py = (y as i32 + dy) as usize;
                    if px < 100 && py < 100 {
                        let idx = py * 100 + px;
                        img.data[idx] = 1.0; // Bright spots
                    }
                }
            }
        }

        // FAST should detect these corners with low threshold
        let keypoints = fast_detector(&img, 50, 9).unwrap();
        assert!(keypoints.len() > 0, "Should detect FAST keypoints with bright corners on dark background");
    }

    #[test]
    fn test_orb_features() {
        let mut img = NativeImageBuffer::new(200, 200, 1);

        // Create pattern with distinct features (similar to FAST test)
        for y in 0..200 {
            for x in 0..200 {
                let idx = y * 200 + x;
                img.data[idx] = 0.0; // Black background
            }
        }

        // Add bright corner features (5x5 bright spots)
        for &(cx, cy) in &[(50, 50), (150, 50), (50, 150), (150, 150), (100, 100)] {
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let x = (cx as i32 + dx) as usize;
                    let y = (cy as i32 + dy) as usize;
                    if x < 200 && y < 200 {
                        let idx = y * 200 + x;
                        img.data[idx] = 1.0; // Bright spots
                    }
                }
            }
        }

        // Test that FAST detects features first
        let fast_keypoints = fast_detector(&img, 20, 9).unwrap();
        println!("FAST detected {} keypoints", fast_keypoints.len());
        assert!(fast_keypoints.len() > 0, "FAST should detect features for ORB to work");

        let (keypoints, descriptors) = orb_detect_and_compute(&img, 20).unwrap();
        assert_eq!(keypoints.len(), descriptors.len());
        assert!(keypoints.len() > 0, "Should detect ORB features");
    }

    #[test]
    fn test_hamming_distance() {
        let desc1 = Descriptor {
            data: vec![0b10101010, 0b01010101],
        };
        let desc2 = Descriptor {
            data: vec![0b10101010, 0b01010101],
        };
        let desc3 = Descriptor {
            data: vec![0b11111111, 0b00000000],
        };

        assert_eq!(hamming_distance(&desc1, &desc2), 0);
        assert!(hamming_distance(&desc1, &desc3) > 0);
    }

    #[test]
    fn test_feature_matching() {
        let desc1 = vec![
            Descriptor {
                data: vec![0b10101010],
            },
            Descriptor {
                data: vec![0b11110000],
            },
        ];

        let desc2 = vec![
            Descriptor {
                data: vec![0b10101011],
            }, // Close to desc1[0]
            Descriptor {
                data: vec![0b00001111],
            },
        ];

        let matches = match_features(&desc1, &desc2, 10);
        assert!(matches.len() > 0, "Should find matches");
    }
}

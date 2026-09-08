//! Mathematical invariants for FEA stress tensors, displacements, and strains.

/// Symmetric 3D Cauchy stress tensor components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressTensor {
    pub sxx: f32,
    pub syy: f32,
    pub szz: f32,
    pub sxy: f32,
    pub syz: f32,
    pub szx: f32,
}

impl StressTensor {
    pub fn new(sxx: f32, syy: f32, szz: f32, sxy: f32, syz: f32, szx: f32) -> Self {
        Self { sxx, syy, szz, sxy, syz, szx }
    }

    /// Calculate equivalent von Mises stress:
    /// sigma_v = sqrt( 0.5 * [ (sxx-syy)^2 + (syy-szz)^2 + (szz-sxx)^2 + 6*(sxy^2 + syz^2 + szx^2) ] )
    pub fn von_mises(&self) -> f32 {
        let diff_xy = self.sxx - self.syy;
        let diff_yz = self.syy - self.szz;
        let diff_zx = self.szz - self.sxx;
        let shear_sum = self.sxy * self.sxy + self.syz * self.syz + self.szx * self.szx;

        let val = 0.5 * (diff_xy * diff_xy + diff_yz * diff_yz + diff_zx * diff_zx + 6.0 * shear_sum);
        if val > 0.0 {
            val.sqrt()
        } else {
            0.0
        }
    }

    /// Calculate hydrostatic (mean) stress:
    /// sigma_m = (sxx + syy + szz) / 3
    pub fn hydrostatic(&self) -> f32 {
        (self.sxx + self.syy + self.szz) / 3.0
    }

    /// Calculate the 3 ordered principal stresses: (sigma_1 >= sigma_2 >= sigma_3)
    /// Solves the characteristic cubic equation analytically via Cardano-Viète method.
    pub fn principal_stresses(&self) -> (f32, f32, f32) {
        let i1 = self.sxx + self.syy + self.szz;
        let i2 = self.sxx * self.syy + self.syy * self.szz + self.szz * self.sxx
            - (self.sxy * self.sxy + self.syz * self.syz + self.szx * self.szx);
        let i3 = self.sxx * (self.syy * self.szz - self.syz * self.syz)
            - self.sxy * (self.sxy * self.szz - self.syz * self.szx)
            + self.szx * (self.sxy * self.syz - self.syy * self.szx);

        let p1 = i1 * i1 - 3.0 * i2;
        if p1 <= 1e-12 {
            // Hydrostatic state
            let mean = i1 / 3.0;
            return (mean, mean, mean);
        }

        let q = i1 / 3.0;
        let p = (p1 / 9.0).sqrt();
        let r = (2.0 * i1 * i1 * i1 - 9.0 * i1 * i2 + 27.0 * i3) / (54.0 * p * p * p);

        // Clamp r to [-1, 1] to avoid float precision issues with acos
        let r_clamped = r.clamp(-1.0, 1.0);
        let phi = (r_clamped.acos()) / 3.0;

        let pi = std::f32::consts::PI;
        let mut s1 = q + 2.0 * p * phi.cos();
        let mut s2 = q + 2.0 * p * (phi + 2.0 * pi / 3.0).cos();
        let mut s3 = q + 2.0 * p * (phi + 4.0 * pi / 3.0).cos();

        // Sort descending: s1 >= s2 >= s3
        if s1 < s2 { std::mem::swap(&mut s1, &mut s2); }
        if s2 < s3 { std::mem::swap(&mut s2, &mut s3); }
        if s1 < s2 { std::mem::swap(&mut s1, &mut s2); }

        (s1, s2, s3)
    }

    /// Maximum shear stress (Tresca criterion):
    /// tau_max = (sigma_1 - sigma_3) / 2
    pub fn max_shear(&self) -> f32 {
        let (s1, _, s3) = self.principal_stresses();
        (s1 - s3) * 0.5
    }
}

/// Calculate displacement Euclidean norm: sqrt(ux^2 + uy^2 + uz^2)
pub fn displacement_magnitude(ux: f32, uy: f32, uz: f32) -> f32 {
    (ux * ux + uy * uy + uz * uz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniaxial_von_mises() {
        let s = StressTensor::new(100.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((s.von_mises() - 100.0).abs() < 1e-4);
        let (s1, s2, s3) = s.principal_stresses();
        assert!((s1 - 100.0).abs() < 1e-4);
        assert!(s2.abs() < 1e-4);
        assert!(s3.abs() < 1e-4);
        assert!((s.max_shear() - 50.0).abs() < 1e-4);
    }

    #[test]
    fn test_pure_shear_von_mises() {
        let s = StressTensor::new(0.0, 0.0, 0.0, 50.0, 0.0, 0.0);
        // Pure shear von Mises is sqrt(3) * tau
        let expected = (3.0_f32).sqrt() * 50.0;
        assert!((s.von_mises() - expected).abs() < 1e-4);
    }

    #[test]
    fn test_displacement_magnitude() {
        assert!((displacement_magnitude(3.0, 4.0, 0.0) - 5.0).abs() < 1e-5);
    }
}

/*
Non-interactive Schnorr ZK DLOG Proof scheme with a Fiat-Shamir transformation
*/

use curv::arithmetic::Converter;
use curv::elliptic::curves::{Point, Scalar, Secp256k1};
use curv::BigInt;
use sha2::Digest;



pub struct DLogProof {
    // Proof values
    pub t: Point<Secp256k1>,
    pub s: Scalar<Secp256k1>,
}

impl DLogProof {
    fn new(t: Point<Secp256k1>, s: Scalar<Secp256k1>) -> Self {
        DLogProof { t, s }
    }

    fn hash_points(sid: &str, pid: i32, points: Vec<Point<Secp256k1>>) -> Scalar<Secp256k1> {
        // Hash the session ID, participant ID, and points
        // Convert the hash result into a BigInt and then into a Scalar
        let mut hasher = sha2::Sha256::new();
        hasher.update(sid.as_bytes());
        hasher.update(&pid.to_be_bytes());
        for point in points {
            hasher.update(point.to_bytes(false).as_ref());
        }
        let hash_result = hasher.finalize();
        let hash_bytes: &[u8] = &hash_result[..];

        // Convert the hash result into a BigInt and then into a Scalar
        let hash_bigint = BigInt::from_bytes(hash_bytes.try_into().unwrap());
        let c = Scalar::<Secp256k1>::from_bigint(&hash_bigint);

        // Ensure the scalar is valid (not zero)
        if c.is_zero() {
            panic!("Hash resulted in zero scalar");

        } else {
            c

        }
    }

    pub fn prove(sid: &str, pid: i32, x: Scalar<Secp256k1>, y: Point<Secp256k1>, base_point: Point<Secp256k1>) -> DLogProof {
        // Function to generate a non-interactive Schnorr ZK DLOG proof 
        // for a given session ID, participant ID, secret x, public y, and base point
        let r = Scalar::random();
        let t = base_point.clone() * r.clone();
        let c = DLogProof::hash_points(sid, pid, vec![base_point.clone(), y.clone(), t.clone()]);

        let s = r + x * c;

        DLogProof::new(t, s)
    }

    pub fn verify(&self, sid: &str, pid: i32, y: Point<Secp256k1>, base_point: Point<Secp256k1>) -> bool {
        // Function to verify a non-interactive Schnorr ZK DLOG proof
        // for a given session ID, participant ID, public y, and base point
        let c = DLogProof::hash_points(sid, pid, vec![base_point.clone(), y.clone(), self.t.clone()]);
        let lhs = base_point * self.s.clone();
        let rhs = self.t.clone() + c * y;

        lhs == rhs
    }
}


//tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlog_proof() {
        let sid = "sid";
        let pid = 1;

        let base_point = Point::generator();
        let x = Scalar::random();
        let y = base_point.clone() * x.clone();

        let dlog_proof = DLogProof::prove(sid, pid, x, y.clone(), base_point.into());
        assert!(dlog_proof.verify(sid, pid, y, base_point.into()));
    }

    #[test]
    fn test_dlog_proof_fail() {
        // Test the DLogProof.verify() function with an incorrect proof
        // The proof should fail because we are adding the base point to y
        let sid = "sid";
        let pid = 1;

        let base_point = Point::generator();
        let x = Scalar::random();
        let y = base_point.clone() * x.clone();

        let dlog_proof = DLogProof::prove(sid, pid, x, y.clone(), base_point.into());
        assert!(!dlog_proof.verify(sid, pid, y.clone() + base_point, base_point.into()));
    }


    #[test]
    fn test_dlog_proof_fail2() {
        // Test the DLogProof.verify() function with an incorrect proof
        // The proof should fail because we are adding a random point to y
        let sid = "sid";
        let pid = 1;

        let base_point = Point::generator();
        let x = Scalar::random();
        let y = base_point.clone() * x.clone();

        let dlog_proof = DLogProof::prove(sid, pid, x, y.clone(), base_point.into());
        assert!(!dlog_proof.verify(sid, pid, y.clone() + base_point.clone() * Scalar::random(), base_point.into()));
    }
}




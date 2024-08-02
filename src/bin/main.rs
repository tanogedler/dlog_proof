
use std::time::Instant;
use dlog_proof::DLogProof;
use curv::elliptic::curves::{Point,Scalar};

fn main() {
    let sid = "sid";
    let pid = 1;

    let base_point = Point::generator();
    let x = Scalar::random();
    let y = base_point * x.clone();

    let start_proof = Instant::now();
    let dlog_proof = DLogProof::prove(sid, pid, x, y.clone(), base_point.into());
    println!("Proof computation time: {:?} ms", start_proof.elapsed().as_millis());

    println!("t: ({:?}, {:?})", dlog_proof.t.x_coord(), dlog_proof.t.y_coord());
    println!("s: {:?}", dlog_proof.s);

    let start_verify = Instant::now();
    let result = dlog_proof.verify(sid, pid, y, base_point.into());
    println!("Verify computation time: {:?} ms", start_verify.elapsed().as_millis());

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
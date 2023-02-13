///M from CLASS - npbparams
const   M : i32 = 36;

const 	MK : i32 = 16;
const 	MM : i32 = M - MK;
const 	NN : i32= 1 << MM;
const	NK : i32= 1 << MK;
const	NQ : i32= 10;
const EPSILON : f64 = 1.0e-8;
const	A : f64 = 1220703125.0;
const	S : f64 = 271828183.0;
const NK_PLUS: i32 = 2*NK+1;

const X : Vec<f64> = Vec::with_capacity(NK_PLUS as usize);

fn main() {
    
    let mut Mops:f64;
    let mut t1	:f64;
    let mut t2	:f64;
    let mut t3	:f64;
    let mut t4	:f64;
    let mut x1	:f64;
    let mut x2	:f64;

    let mut sx	:f64;
    let mut sy	:f64;
    let mut tm	:f64;
    let mut an	:f64;
    let mut tt	:f64;
    let mut gc	:f64;
    
    let mut sx_verify_value	:f64;
    let mut sy_verify_value	:f64;
    let mut sx_err	:f64;
    let mut sy_err	:f64;
    
    let mut np :i32;

    let mut i :i32;
    let mut ik :i32;
    let mut kk :i32;
    let mut l :i32;
    let mut k :i32;
    let mut nit :i32;

    let mut k_offset :i32;
    let mut j :i32;

    let mut verified :bool;
    let mut timers_enabled :bool;
    
    let mut dum = [1.0, 1.0, 1.0];

    verified = false;

    np = NN;

    
}

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


const R23 :f64 = 0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5;
const R46 :f64 = R23*R23;
const T23 :f64 = 2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0;
const T46 :f64 = T23*T23;


pub mod randdp;

use crate::randdp::*;
fn main() {
    
    //= Vec::new();//
    let mut x : Vec<f64> = Vec::with_capacity(NK_PLUS as usize);
    let mut q : Vec<f64> = Vec::with_capacity(NQ as usize);
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

    //let mut i :usize;
    let mut ik :i32;
    let mut kk :i32;
    let mut l :i32;
    let mut k :i32;
    let mut nit :i32;

    let mut k_offset :i32;
    let mut j :i32;

    let mut verified :bool;
    let mut timers_enabled :bool;
    
    //let mut dum = [1.0, 1.0, 1.0];
    let mut dum: Vec<f64> = [1.0, 1.0, 1.0].to_vec();

    verified = false;

    np = NN;
    //vranlc(0, &mut dum[0],  dum[1], &mut dum);

    t1 = A;
    vranlc(0, &mut t1, A, &mut x);
    
    t1 = A;
    let mut t1_1: f64;
    for i in 0..MK+1 {
        t1_1 = t1;
        t2 = randlc(&mut t1, t1_1)
    }

    an = t1;
	tt = S;
	gc = 0.0;
	sx = 0.0;
	sy = 0.0;

    for i in 0..NQ as usize {
        q[i] = 0.0;
    }

    k_offset = -1;

    let mut t2_2: f64;
    for k in 1..np+1 {
        kk = k_offset + k;
		t1 = S;
		t2 = an;

        for i in 1..101 {
            ik = kk / 2;
			if (2*ik)!=kk{
                t3=randlc(&mut t1,t2);
            }
			if ik==0{
                break;
            }
            t2_2 = t2;
			t3=randlc(&mut t2,t2_2);
			kk=ik;
        }

        //if(timers_enabled){timer_start(2);}
		vranlc(2*NK as usize, &mut t1, A, &mut x);
		//if(timers_enabled){timer_stop(2);}
        
        //if(timers_enabled){timer_start(1);}
        
        for i in 0..NK {
			x1 = 2.0 * x[2*i as usize] - 1.0;
			x2 = 2.0 * x[(2*i+1) as usize] - 1.0;
			t1 = x1*x1 + x2*x2;
			//if(t1 <= 1.0){
				//t2 = sqrt(-2.0 * log(t1) / t1);
				//t3 = (x1 * t2);
				//t4 = (x2 * t2);
				//l = max(fabs(t3), fabs(t4));
				//q[l] += 1.0;
				//sx = sx + t3;
				//sy = sy + t4;
			//}
		}
		//if(timers_enabled){timer_stop(1);}

    }
}

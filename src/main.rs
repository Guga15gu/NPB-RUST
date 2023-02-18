///M from CLASS - npbparams
const   M : i32 = 24;
const 	MK : i32 = 16;
const 	MM : i32 = M - MK;
const 	NN : i32= 1 << MM;
const	NK : i32= 1 << MK;
const	NQ : i32= 10;
const EPSILON : f64 = 1.0e-8;
const	A : f64 = 1220703125.0;
const	S : f64 = 271828183.0;
const NK_PLUS: i32 = 2*NK+1;

pub mod randdp;
use crate::randdp::*;


fn main() {
    
    //= Vec::new();//
    //let mut x : Vec<f64> = Vec::with_capacity(NK_PLUS as usize);
    let mut x: Vec<f64> = vec!(0.0; NK_PLUS as usize);
    //let mut q : Vec<f64> = Vec::with_capacity(NQ as usize);
    let mut q: Vec<f64> = vec!(0.0; NQ as usize);
    let mut mops:f64;
    let mut t1	:f64;
    let mut t2	:f64;
    let mut t3	:f64;
    let mut t4	:f64;
    let mut x1	:f64;
    let mut x2	:f64;

    let mut sx	:f64;
    let mut sy	:f64;
    let mut tm	:f64;
    let an	:f64;
    let tt	:f64;
    let mut gc	:f64;
    
    let sx_verify_value	:f64;
    let sy_verify_value	:f64;
    let sx_err	:f64;
    let sy_err	:f64;
    
    let np :i32;

    //let mut i :usize;
    let mut ik :i32;
    let mut kk :i32;
    let mut l :i32;
    let mut k :i32;
    let nit :i32;

    let k_offset :i32;
    let mut j :i32;

    let mut verified :bool;
    let mut timers_enabled :bool;
    
    //let mut dum = [1.0, 1.0, 1.0];
    let dum: Vec<f64> = [1.0, 1.0, 1.0].to_vec();
    //char    size[16];

	//FILE* fp;
	//if((fp = fopen("timer.flag", "r"))==NULL){
	//	timers_enabled = FALSE;
	//}else{
	//	timers_enabled = TRUE;
	//	fclose(fp);
	//}
    
    /*
	 * --------------------------------------------------------------------
	 * because the size of the problem is too large to store in a 32-bit
	 * integer for some classes, we put it into a string (for printing).
	 * have to strip off the decimal point put in there by the floating
	 * point print statement (internal file)
	 * --------------------------------------------------------------------
	 */
    //sprintf(size, "%15.0f", pow(2.0, M+1));
	//j = 14;
	//if(size[j]=='.'){j--;}
	//size[j+1] = '\0';
	//printf("\n\n NAS Parallel Benchmarks 4.1 Serial C++ version - EP Benchmark\n\n");
	//printf(" Number of random numbers generated: %15s\n", size);
    
    //verified = false;

    /*
	 * --------------------------------------------------------------------
	 * compute the number of "batches" of random number pairs generated 
	 * per processor. Adjust if the number of processors does not evenly 
	 * divide the total number
	 * --------------------------------------------------------------------
	 */
    np = NN;

    /*
	 * call the random number generator functions and initialize
	 * the x-array to reduce the effects of paging on the timings.
	 * also, call all mathematical functions that are used. make
	 * sure these initializations cannot be eliminated as dead code.
	 */
    //vranlc(0, &mut dum[0],  dum[1], &mut dum);
    //vranlc(0, &dum[0], dum[1], &dum[2]);
	//dum[0] = randlc(&dum[1], dum[2]);
	//for(i=0; i<NK_PLUS; i++){x[i] = -1.0e99;}
	//mops = log(sqrt(fabs(max(1.0, 1.0))));
    
    //timer_clear(0);
	//timer_clear(1);
	//timer_clear(2);
	//timer_start(0);
    
    t1 = A;
    vranlc(0, &mut t1, A, &mut x);
    
    /* compute AN = A ^ (2 * NK) (mod 2^46) */

    t1 = A;
    
    let mut t1_1: f64;
    for _i in 0..MK+1 {
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

    /*
	 * each instance of this loop may be performed independently. we compute
	 * the k offsets separately to take into account the fact that some nodes
	 * have more numbers to generate than others
	 */
    k_offset = -1;

    let mut t2_2: f64;
    for k in 1..np+1 {
        kk = k_offset + k;
		t1 = S;
		t2 = an;

        /* find starting seed t1 for this kk */
        for _i in 1..101 {
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

        /* compute uniform pseudorandom numbers */
        //if(timers_enabled){timer_start(2);}
		vranlc(2*NK as usize, &mut t1, A, &mut x);
		//if(timers_enabled){timer_stop(2);}
        
        /*
		 * compute gaussian deviates by acceptance-rejection method and
		 * tally counts in concentric square annuli. this loop is not
		 * vectorizable.
		 */

        //if(timers_enabled){timer_start(1);}
        
        for i in 0..NK {
			x1 = 2.0 * x[2*i as usize] - 1.0;
			x2 = 2.0 * x[(2*i+1) as usize] - 1.0;
			t1 = x1*x1 + x2*x2;
			if t1 <= 1.0{
				t2 = (-2.0 * t1.ln() / t1).sqrt();
				t3 = x1 * t2;
				t4 = x2 * t2;
				l = (t3.abs()).max(t4.abs()) as i32;
				q[l as usize] += 1.0;
				sx = sx + t3;
				sy = sy + t4;
			}
		}
		//if(timers_enabled){timer_stop(1);}
    }
    
    for i in 0..NQ as usize {
		gc = gc + q[i];
	}
    
	//timer_stop(0);
	//tm = timer_read(0);

	nit = 0;
	verified = true;
	if M == 24{
		sx_verify_value = -3.247834652034740e+3;
		sy_verify_value = -6.958407078382297e+3;
	}else if M == 25 {
		sx_verify_value = -2.863319731645753e+3;
		sy_verify_value = -6.320053679109499e+3;
	}else if M == 28 {
		sx_verify_value = -4.295875165629892e+3;
		sy_verify_value = -1.580732573678431e+4;
	}else if M == 30 {
		sx_verify_value =  4.033815542441498e+4;
		sy_verify_value = -2.660669192809235e+4;
	}else if M == 32 {
		sx_verify_value =  4.764367927995374e+4;
		sy_verify_value = -8.084072988043731e+4;
	}else if M == 36 {
		sx_verify_value =  1.982481200946593e+5;
		sy_verify_value = -1.020596636361769e+5;
	}else if M == 40 {
		sx_verify_value = -5.319717441530e+05;
		sy_verify_value = -3.688834557731e+05;
	}else{
		verified = false;
        sx_verify_value = 0.0;
		sy_verify_value = 0.0;
	}
     
	if verified{
		sx_err = (sx - sx_verify_value).abs() / sx_verify_value;
		sy_err = (sy - sy_verify_value).abs() / sy_verify_value;
		verified = (sx_err <= EPSILON) && (sy_err <= EPSILON);
	}
    
	//mops = f64::powi(2.0, M+1)/tm/1000000.0;
    print!("\n EP Benchmark Results:\n\n");
	//print!(" CPU Time ={%10.4f}\n", tm);
	
    print!(" N = 2^{}\n", M);
	print!(" No. Gaussian Pairs = {}\n", gc);
     
	print!(" Sums = {}e {}e\n", sx, sy);
	print!(" Counts: \n");
    
	for i in 0..(NQ-1) as usize {
		print!("{}d{}f\n", i, q[i]);
	}

    print!(" Verified: {} \n", verified);
    /*
	c_print_results((char*)"EP",
			CLASS,
			M+1,
			0,
			0,
			nit,
			tm,
			Mops,
			(char*)"Random numbers generated",
			verified,
			(char*)NPBVERSION,
			(char*)COMPILETIME,
			(char*)COMPILERVERSION,
			(char*)CS1,
			(char*)CS2,
			(char*)CS3,
			(char*)CS4,
			(char*)CS5,
			(char*)CS6,
			(char*)CS7);

	if(timers_enabled){
		if(tm <= 0.0){tm = 1.0;}
		tt = timer_read(0);
		printf("\nTotal time:     %9.3f (%6.2f)\n", tt, tt*100.0/tm);
		tt = timer_read(1);
		printf("Gaussian pairs: %9.3f (%6.2f)\n", tt, tt*100.0/tm);
		tt = timer_read(2);
		printf("Random numbers: %9.3f (%6.2f)\n", tt, tt*100.0/tm);
	}

    return 0;
    */
}

const T_BENCHMARKING    :u32 = 0;
const T_INITIALIZATION  :u32 = 1;
const T_SORTING         :u32 = 2;
const T_TOTAL_EXECUTION :u32 = 3;

const USE_BUCKETS       :bool = true;

//#[cfg (not(CLASS))]
//const CLASS :String = "S".to_string();
/* 
#[cfg(feature = "class_S")]
const TOTAL_KEYS_LOG_2:       i32 = 16;
#[cfg(feature = "class_S")]
const MAX_KEY_LOG_2:          i32 = 11;
#[cfg(feature = "class_S")]
const NUM_BUCKETS_LOG_2:      i32 = 9;
#[cfg(feature = "class_S")]
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;

#[cfg(feature = "class_W")]
const TOTAL_KEYS_LOG_2:       i32 = 20;
#[cfg(feature = "class_W")]
const MAX_KEY_LOG_2:          i32 = 16;
#[cfg(feature = "class_W")]
const NUM_BUCKETS_LOG_2:      i32 = 10;
#[cfg(feature = "class_W")]
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;

#[cfg(feature = "class_A")]
const TOTAL_KEYS_LOG_2:       i32 = 23;
#[cfg(feature = "class_A")]
const MAX_KEY_LOG_2:          i32 = 19;
#[cfg(feature = "class_A")]
const NUM_BUCKETS_LOG_2:      i32 = 10;
#[cfg(feature = "class_A")]
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;

#[cfg(feature = "class_B")]
const TOTAL_KEYS_LOG_2:       i32 = 25;
#[cfg(feature = "class_B")]
const MAX_KEY_LOG_2:          i32 = 21;
#[cfg(feature = "class_B")]
const NUM_BUCKETS_LOG_2:      i32 = 10;
#[cfg(feature = "class_B")]
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;

#[cfg(feature = "class_C")]
const TOTAL_KEYS_LOG_2:       i32 = 27;
#[cfg(feature = "class_C")]
const MAX_KEY_LOG_2:          i32 = 23;
#[cfg(feature = "class_C")]
const NUM_BUCKETS_LOG_2:      i32 = 10;
#[cfg(feature = "class_C")]
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;

#[cfg(feature = "class_D")]
const TOTAL_KEYS_LOG_2:       i32 = 31;
#[cfg(feature = "class_D")]
const MAX_KEY_LOG_2:          i32 = 27;
#[cfg(feature = "class_D")]
const NUM_BUCKETS_LOG_2:      i32 = 10;
#[cfg(feature = "class_D")]
const TOTAL_KEYS: i64 = 1L << TOTAL_KEYS_LOG_2;

#[cfg(feature = "CLASS")]
const MAX_KEY:                  i32 = 1 << MAX_KEY_LOG_2;
#[cfg(feature = "CLASS")]
const NUM_BUCKETS:              i32 = 1 << NUM_BUCKETS_LOG_2;
#[cfg(feature = "CLASS")]
const NUM_KEYS:                 i32 = TOTAL_KEYS;
#[cfg(feature = "CLASS")]
const SIZE_OF_BUFFERS:          i32 = NUM_KEYS;
*/

const TOTAL_KEYS_LOG_2:       i32 = 16;
const MAX_KEY_LOG_2:          i32 = 11;
const NUM_BUCKETS_LOG_2:      i32 = 9;
const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
const MAX_ITERATIONS:i32 = 10;
const TEST_ARRAY_SIZE:i32 = 5;

const MAX_KEY:                  i32 = 1 << MAX_KEY_LOG_2;
const NUM_BUCKETS:              i32 = 1 << NUM_BUCKETS_LOG_2;
const NUM_KEYS:                 i32 = TOTAL_KEYS;
const SIZE_OF_BUFFERS:          i32 = NUM_KEYS;

#[cfg(CLASS = "D")]
type IntType = i64;
#[cfg(not(CLASS = "D"))]
type IntType = i32;

pub mod randdp;
pub mod timers;
use crate::randdp::*;
use crate::timers::*;

use rayon::prelude::*;
use rayon::current_num_threads;
///*
//#[cfg(feature = "CLASS")]
fn create_seq(seed: f64, an:f64, v: &mut Vec<IntType>){

    let num_threads = current_num_threads();
    let num_procs = num_threads as i32;

    //Aqui irei apenas dividir o vetor em um pra cada thread,
    //assim o compilador entende que tá tudo certo com os acessos
    //em caso de divisao n exata, um chunk extra vai ser criado
    //o que vai ser uma iteracao a mais, seria bom juntar com a última...
    /* 
    let mq :IntType;
    mq = (NUM_KEYS + num_procs - 1) / num_procs;
    k1 = mq * myid;
    k2 = k1 + mq;
    if k2 > NUM_KEYS{
        k2 = NUM_KEYS;
    } 
    */
    //let key_arrays = v.chunks_mut(num_threads);
    //let ideia : Vec<i32> = (0..num_threads as i32).collect();

	let iterator = v.par_chunks_mut(num_threads).enumerate().for_each(| (myid, key_array)|
    //#pragma omp parallel
	{
		let mut x :f64 = 0.0;
        let mut s :f64;

		let i :IntType;
        let k :IntType;

		let k1 :IntType;
        let mut k2 :IntType;
		
        //pra que isso?
        //double an = a;
		
        
		s = find_my_seed( myid as i32, 
				num_procs,
	            4*(NUM_KEYS as i64),
				seed,
				an );

		k = MAX_KEY/4;
        
		//for(i=k1; i<k2; i++){
        //for i in k1..k2{
        key_array.iter_mut().for_each(|pos|{
            x = randlc(&mut s, an);
			x += randlc(&mut s, an);
			x += randlc(&mut s, an);
			x += randlc(&mut s, an);
            *pos = k * (x as i32);
			//key_array[i as usize] = k * x as i32;
        });
			

	}); /*omp parallel*/
    
}

fn find_my_seed(kn: i32, np: i32, nn: i64, s:f64, a: f64) -> f64{
    let mut t1 :f64;
    let mut t2 :f64;
    let mq :i64;
    let nq :i64;
    let mut kk :i64;
    let mut ik :i64;

    if kn == 0 {
        return s;
    } 

    mq = (((nn as f64)/4.0 + (np as f64) - 1.0) / (np as f64)).abs() as i64;
	nq = ((mq as f64) * 4.0 * (kn as f64)).abs() as i64; /* number of rans to be skipped */

    t1 = s;
	t2 = a;
	kk = nq;

    while kk > 1 {
		ik = kk / 2;
		if 2 * ik ==  kk {
            let t2_copy = t2;
			randlc( &mut t2, t2_copy);
			kk = ik;
		}
		else{
			randlc( &mut t1, t2 );
			kk = kk - 1;
		}
	}

    randlc( &mut t1, t2 );

    t1
}
//*/
fn main() {

    let mut key_array: Vec<IntType> = vec![0; SIZE_OF_BUFFERS as usize];
    let mut key_buff1: Vec<IntType>;
    let mut key_buff2: Vec<IntType>;
    let mut partial_verify_vals: Vec<IntType>;
    let mut key_buff1_aptr: Vec<IntType>;

    //#[cfg(feature = "CLASS")]
    create_seq(314159265.00, 1220703125.00, &mut key_array);

    key_array.iter().for_each(|x|{
        println!("{}", *x);
    })
}
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
const CLASS: char = 'S';
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

const S_test_index_array : [IntType; 5] = [48427,17148,23627,62548,4431];
const S_test_rank_array : [IntType; 5] = [0,18,346,64917,65463];

const W_test_index_array : [IntType; 5] = [357773,934767,875723,898999,404505];
const W_test_rank_array : [IntType; 5] = [1249,11698,1039987,1043896,1048018];

const A_test_index_array : [IntType; 5] = [2112377,662041,5336171,3642833,4250760];
const A_test_rank_array : [IntType; 5] = [104,17523,123928,8288932,8388264];

const B_test_index_array : [IntType; 5] = [41869,812306,5102857,18232239,26860214];
const B_test_rank_array : [IntType; 5] = [33422937,10244,59149,33135281,99];

const C_test_index_array : [IntType; 5] = [44172927,72999161,74326391,129606274,21736814];
const C_test_rank_array : [IntType; 5] = [61147,882988,266290,133997595,133525895];

const D_test_index_array : [IntType; 5] = [1317351170,995930646,1157283250,1503301535,1453734525];
const D_test_rank_array : [IntType; 5] = [1,36538729,1978098519,2145192618,2147425337];

#[cfg(CLASS = "D")]
type IntType = i64;
#[cfg(not(CLASS = "D"))]
type IntType = i32;

pub mod randdp;
pub mod timers;
use crate::randdp::*;
use crate::timers::*;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use rayon::prelude::*;
use rayon::current_num_threads;


fn create_seq(seed: f64, a:f64, v: &mut Vec<IntType>){

    //let num_threads = current_num_threads();
    //let num_procs = num_threads as i32;
    let num_procs = 1;
    let myid = 0;
    //Aqui irei apenas dividir o vetor em um pra cada thread,
    //assim o compilador entende que t?? tudo certo com os acessos
    //em caso de divisao n exata, um chunk extra vai ser criado
    //o que vai ser uma iteracao a mais, seria bom juntar com a ??ltima...
    
    let mq :IntType;
    mq = (NUM_KEYS + num_procs - 1) / num_procs;
    /* 
    k1 = mq * myid;
    k2 = k1 + mq;
    if k2 > NUM_KEYS{
        k2 = NUM_KEYS;
    } 
    */
    //let key_arrays = v.chunks_mut(num_threads);
    //let ideia : Vec<i32> = (0..num_threads as i32).collect();

	//let iterator = v.par_chunks_mut(mq as usize).enumerate().for_each(| (myid, key_array)|
    //#pragma omp parallel
	//{
		let mut x :f64 = 0.0;
        let mut s :f64;

		let i :IntType;
        let k :IntType;

		let k1 :IntType;
        let mut k2 :IntType;
		
        //pra que isso?
        let an = a;
		
        
		s = find_my_seed( myid as i32, 
				num_procs,
	            4*(NUM_KEYS as i64),
				seed,
				an );
        
        //println!("{} s", s);
		k = MAX_KEY/4;
        
		//for(i=k1; i<k2; i++){
        //for i in k1..k2{
        v.iter_mut().for_each(|pos|{
            x = randlc(&mut s, an);
			x += randlc(&mut s, an);
			x += randlc(&mut s, an);
			x += randlc(&mut s, an);
            //println!("{} x", x);
            //println!("{} k", k);
            //println!("{} k*x", ((k as f64) * x ) as i32);
            *pos = ((k as f64) * x ) as i32;
            //println!("{} key_array", *pos);
			//key_array[i as usize] = k * x as i32;
        });
			

	//}); /*omp parallel*/
    
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

fn alloc_key_buff () -> (Vec<Vec<IntType>>, Vec<IntType>) {
    let i: IntType;
	//let num_procs = current_num_threads();
    let num_procs = 1;
    
    let mut bucket_size: Vec<Vec<IntType>> = vec![vec![0;NUM_BUCKETS as usize]; num_procs];
    
    /*
    let mut bucket_size: Vec<&mut Vec<IntType>> = Vec::new();

    for i in 0..num_procs{
        let mut vec_temp = vec![0;NUM_BUCKETS as usize];
        bucket_size.push(&mut vec_temp);
    }
    */
    let mut key_buff2: Vec<IntType> = vec![0;NUM_KEYS as usize];

    /*
    #pragma omp parallel for
    for( i=0; i<NUM_KEYS; i++ )
        key_buff2[i] = 0;
    
    */
    (bucket_size, key_buff2)
}

fn full_verify(bucket_ptrs: &mut Vec<IntType>, 
    key_array: &mut Vec<IntType>, 
    passed_verification: &mut i32,
    key_buff_ptr_global: &mut Vec<IntType>,
    key_buff2: &Vec<IntType>) {
    
    let i :IntType;
    let mut j :IntType;

    let mut k :IntType;
    let mut k1 :IntType;
    let k2 :IntType;

    let myid :usize = 0;
    let num_procs :usize = 1;

    //#pragma omp parallel for private(i,j,k,k1) schedule(dynamic)
    for j in 0..NUM_BUCKETS as usize{
        if j>0 {
            k1 = bucket_ptrs[j-1];
        }
        else{
            k1 = 0;
        }
	
        for i in k1..bucket_ptrs[j]{
            key_buff_ptr_global[key_buff2[i as usize] as usize] -= 1;
			k = key_buff_ptr_global[key_buff2[i as usize] as usize];
			key_array[k as usize] = key_buff2[i as usize];
		}
	}

    j = 0;
	//#pragma omp parallel for reduction(+:j)
    for i in 1..NUM_KEYS as usize{
		if key_array[i-1] > key_array[i] {
			j += 1;
        }
    }
	if j != 0 {
		println!( "Full_verify: number of keys out of sort: {}", j );
    }
	else{
		*passed_verification += 1;
    }

}

fn rank<'a> (iteration: i32, 
        key_array: &mut Vec<IntType>, 
        partial_verify_vals: &mut Vec<IntType>,
        key_buff2: &mut Vec<IntType>,
        key_buff1: &'a mut Vec<IntType>,
        bucket_size: &mut Vec<Vec<IntType>>,
        bucket_ptrs: &mut Vec<IntType>,
        test_index_array: [IntType; 5],
        test_rank_array: [IntType; 5]) -> i32
        /*mut key_buff_ptr_global: &'a mut Vec<IntType>)*/{
            
    let i :IntType;
    let mut k :IntType;

    let mut key_buff_ptr: &mut Vec<IntType>;
    //let mut key_buff_ptr2: &mut Vec<IntType>;

    let shift:i32=MAX_KEY_LOG_2-NUM_BUCKETS_LOG_2;

    let num_bucket_keys:IntType= 1<<shift;

    key_array[iteration as usize] = iteration;
	key_array[(iteration+MAX_ITERATIONS) as usize] = MAX_KEY - iteration;

	for i in 0..TEST_ARRAY_SIZE as usize{
        partial_verify_vals[i] = key_array[test_index_array[i] as usize];
    }
    
    //key_buff_ptr2 = key_buff2;
    key_buff_ptr = key_buff1;

    
    //let work_buff: &mut Vec<IntType>;
    let mut m: IntType;
    let mut k1: IntType;
    let mut k2: IntType;

    let myid = 0;
    let num_procs = 1;

    let work_buff = &mut bucket_size[myid];
    
    /* Initialize */
    /* 
    //j?? foi feito isso na funcao alloc_key_buff
    for( i=0; i<NUM_BUCKETS; i++ )  
        work_buff[i] = 0;
    */
    work_buff.iter_mut().for_each(|a| {*a = 0});
    /* Determine the number of keys in each bucket */
    //#pragma omp for schedule(static)
    for i in 0..NUM_KEYS{
        work_buff[(key_array[i as usize] >> shift) as usize]+= 1;
    }
    
    /* Accumulative bucket sizes are the bucket pointers. */
    /* These are global sizes accumulated upon to each bucket */
    bucket_ptrs[0] = 0;
    for k in 0..myid { 
        bucket_ptrs[0] += bucket_size[k][0];
    } 
    
    for i in 1..NUM_BUCKETS as usize { 
        bucket_ptrs[i] = bucket_ptrs[i-1];
        for k in 0..myid{
            bucket_ptrs[i] += bucket_size[k][i];
        }
        for k in myid..num_procs{
            bucket_ptrs[i] += bucket_size[k][i-1];
        }
    }
    
    /* Sort into appropriate bucket */
    //#pragma omp for schedule(static)
    for i in 0..NUM_KEYS as usize{
        k = key_array[i];
        //println!("k = {}", k);
        //println!("k >> shift = {}", k >> shift);
        //println!("bucket_ptrs[k >> shift] = {}", bucket_ptrs[(k >> shift) as usize]);
        key_buff2[(bucket_ptrs[(k >> shift) as usize]) as usize] = k;
        bucket_ptrs[(k >> shift) as usize] += 1;
    }
    
    /* The bucket pointers now point to the final accumulated sizes */
    if myid < num_procs-1 {
        for i in 0..NUM_BUCKETS as usize{
            for k in myid+1..num_procs{
                bucket_ptrs[i] += bucket_size[k][i];
            }
        }
    }
    
    /* Now, buckets are sorted.  We only need to sort keys inside */
    /* each bucket, which can be done in parallel.  Because the distribution */
    /* of the number of keys in the buckets is Gaussian, the use of */
    /* a dynamic schedule should improve load balance, thus, performance */
    //#pragma omp for schedule(dynamic)
    for i in 0..NUM_BUCKETS{
        /* Clear the work array section associated with each bucket */
        k1 = i * num_bucket_keys;
        k2 = k1 + num_bucket_keys;
        for k in k1..k2{
            key_buff_ptr[k as usize] = 0;
        }
        /* Ranking of all keys occurs in this section: */
        /* In this section, the keys themselves are used as their */
        /* own indexes to determine how many of each there are: their */
        /* individual population */
        if i > 0 {
            m = bucket_ptrs[(i-1) as usize];
        }
        else{
            m = 0;
        }
        for k in m..bucket_ptrs[i as usize]{
            //key_buff_ptr[key_buff_ptr2[k as usize] as usize]+= 1;
            key_buff_ptr[key_buff2[k as usize] as usize]+= 1;
        } 
        /* Now they have individual key population */
        /* To obtain ranks of each key, successively add the individual key */
        /* population, not forgetting to add m, the total of lesser keys, */
        /* to the first key population */
        key_buff_ptr[k1 as usize] += m;
        for k in k1+1..k2{
            key_buff_ptr[k as usize] += key_buff_ptr[(k-1) as usize];
        }
    }
    
    /* This is the partial verify test section */
    /* Observe that test_rank_array vals are */
    /* shifted differently for different cases */
    let mut passed_verification = 0;

    for i in 0..TEST_ARRAY_SIZE as usize{                                             
        k = partial_verify_vals[i]; /* test vals were put here */
        if 0 < k  &&  k <= NUM_KEYS-1 {
    
            let key_rank = key_buff_ptr[(k-1) as usize];
            let mut failed = 0;

            match CLASS {
                'S' => {
                    if i <= 2 {
                        if key_rank != test_rank_array[i] + iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                'W' => {
                    if i < 2 {
                        if key_rank != test_rank_array[i] + (iteration - 2) {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                'A' => {
                    if i <= 2 {
                        if key_rank != test_rank_array[i] + (iteration - 1) {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - (iteration - 1) {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                'B' => {
                    if i == 1 || i == 2 || i == 4 {
                        if key_rank != test_rank_array[i] + iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                'C' => {
                    if i <= 2 {
                        if key_rank != test_rank_array[i] + iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                'D' => {
                    if i < 2 {
                        if key_rank != test_rank_array[i] + iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    } else {
                        if key_rank != test_rank_array[i] - iteration {
                            failed = 1;
                        } else {
                            passed_verification += 1;
                        }
                    }
                },
                _ => { /* caso padr??o */ }
            }
            
            if failed == 1 {
                println!( "Failed partial verification:\niteration {}, test key {}\n", 
                        iteration, i );
            }
        }   
    }
    
    /* Make copies of rank info for use by full_verify: these variables */
    /* in rank are local; making them global slows down the code, probably */
    /* since they cannot be made register by compiler */
    if iteration == MAX_ITERATIONS {
        //key_buff_ptr_global =  &mut key_buff_ptr;
    }
    passed_verification
}

fn main() {

    let mut key_array: Vec<IntType> = vec![0; SIZE_OF_BUFFERS as usize];
    let mut key_buff1: Vec<IntType> = vec![0; MAX_KEY as usize];
    let mut key_buff2: Vec<IntType> = vec![0; SIZE_OF_BUFFERS as usize];
    let mut partial_verify_vals: Vec<IntType> = vec![0; TEST_ARRAY_SIZE as usize];
    let mut key_buff1_aptr: Vec<IntType>;

    let mut bucket_ptrs = vec![0; NUM_BUCKETS as usize];
    let mut bucket_size: Vec<Vec<i32>>;

    let mut key_buff_ptr_global: Vec<IntType> = vec![];
    let mut passed_verification: i32 = 0;

    let mut iteration;

    let mut start: [Instant; 64] = [Instant::now(); 64];
    let mut elapsed: [Duration; 64] = [Duration::ZERO; 64];
    let tm: f64;

    println!("SIZE_OF_BUFFERS: {}", SIZE_OF_BUFFERS);
    println!("MAX_KEY: {}", MAX_KEY);
    println!("NUM_BUCKETS: {}", NUM_BUCKETS);
    println!("NUM_KEYS: {}", NUM_KEYS);
    println!("MAX_ITERATIONS: {}", MAX_ITERATIONS);

    let mut test_index_array : [IntType; 5] = [48427,17148,23627,62548,4431];
    let mut test_rank_array : [IntType; 5] = [48427,17148,23627,62548,4431];

    for i in 0..TEST_ARRAY_SIZE as usize {
        match CLASS {
            'S' => {
                test_index_array[i] = S_test_index_array[i];
                test_rank_array[i] = S_test_rank_array[i];
            },
            'A' => {
                test_index_array[i] = A_test_index_array[i];
                test_rank_array[i] = A_test_rank_array[i];
            },
            'W' => {
                test_index_array[i] = W_test_index_array[i];
                test_rank_array[i] = W_test_rank_array[i];
            },
            'B' => {
                test_index_array[i] = B_test_index_array[i];
                test_rank_array[i] = B_test_rank_array[i];
            },
            'C' => {
                test_index_array[i] = C_test_index_array[i];
                test_rank_array[i] = C_test_rank_array[i];
            },
            'D' => {
                test_index_array[i] = D_test_index_array[i];
                test_rank_array[i] = D_test_rank_array[i];
            },
            _ => panic!("Invalid CLASS value")
        };
    }

    create_seq(314159265.00, 1220703125.00, &mut key_array);
    
    (bucket_size, key_buff2) = alloc_key_buff();

    iteration = 1;
    /* 
    rank( iteration, 
        &mut key_array, 
        &mut partial_verify_vals,
        &mut key_buff2,
        &mut key_buff1,
        &mut bucket_size,
        &mut bucket_ptrs,
        test_index_array,
        test_rank_array,
        /*&mut key_buff_ptr_global */
        );  
    */
    let mut passed;
    timer_start(0, &mut start);
    
    
    for i in 1..MAX_ITERATIONS+1 {
        iteration = i;
		//if CLASS != 'S'{
            print!("        {}", iteration);
        //}
		passed = rank( iteration, 
            &mut key_array, 
            &mut partial_verify_vals,
            &mut key_buff2,
            &mut key_buff1,
            &mut bucket_size,
            &mut bucket_ptrs,
            test_index_array,
            test_rank_array,
            /*&mut key_buff_ptr_global*/
            );
        println!(", passed: {}", passed);
	}

    timer_stop(0, &mut start, &mut elapsed);
	tm = timer_read(0, &mut elapsed).as_secs_f64();
    
    /*
    full_verify(&mut bucket_ptrs, 
        &mut key_array, 
        &mut passed_verification,
        &mut key_buff_ptr_global,
        &key_buff2);
     */
    if passed_verification != 5*MAX_ITERATIONS + 1{
        passed_verification = 0;
    }
    println!("full_verification: {}", passed_verification);
    /* 
    key_array.iter().enumerate().for_each(|(i, pos)|{
        println!("{}: {}", i, pos);
    });
    */
    print!(" CPU Time ={}\n", tm);
    //let result = find_my_seed(7, 12, 20, 3534.43534, 3244123.234234);
    //println!("{}", result);
    /* 
    key_array.iter().for_each(|x|{
        println!("{}", *x);
    })
    */
}
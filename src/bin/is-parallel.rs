const T_BENCHMARKING    :u32 = 0;
const T_INITIALIZATION  :u32 = 1;
const T_SORTING         :u32 = 2;
const T_TOTAL_EXECUTION :u32 = 3;

const USE_BUCKETS       :bool = true;

//#[cfg (not(CLASS))]
//const CLASS :String = "S".to_string();

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

const MAX_ITERATIONS:i32 = 10;
const TEST_ARRAY_SIZE:i32 = 5;

#[cfg(CLASS = "D")]
type IntType = i64;
#[cfg(not(CLASS = "D"))]
type IntType = i32;

///*
pub fn create_seq(seed: f64, an:f64){

    //#pragma omp parallel
	{
		let x :f64;
        let s :f64;

		let i :i32;
        let k :i32;

		let k1 :i32;
        let mut k2 :i32;
		
        //pra que isso?
        //double an = a;
		let myid :i32 = 0;
        let num_procs :i32 = 1;

		let mq :i32;

		//myid = omp_get_thread_num();
		//num_procs = omp_get_num_threads();

		mq = (NUM_KEYS + num_procs - 1) / num_procs;
		k1 = mq * myid;
		k2 = k1 + mq;
		if k2 > NUM_KEYS{
            k2 = NUM_KEYS;
        } 
        /* 
		s = find_my_seed( myid, 
				num_procs,
				(long)4*NUM_KEYS,
				seed,
				an );

		k = MAX_KEY/4;

		for(i=k1; i<k2; i++){
			x = randlc(&s, an);
			x += randlc(&s, an);
			x += randlc(&s, an);
			x += randlc(&s, an);
			key_array[i] = k*x;
		}
    */
	} /*omp parallel*/
    
}
//*/
fn main() {

    create_seq(3.5, 1.2);
}
use std::time::Instant;
use std::time::Duration;

//pub static mut start: [f64; 64] = [0.0; 64];
//pub static mut elapsed: [f64; 64] = [0.0; 64];

/* 
fn wtime(t: &mut Instant){
    *t = Instant::now();
}
*/
/*****************************************************************/
/******         E  L  A  P  S  E  D  _  T  I  M  E          ******/
/*****************************************************************/

pub fn elapsed_time() -> Instant{
	let t = Instant::now();
	t
}

/*****************************************************************/
/******            T  I  M  E  R  _  C  L  E  A  R          ******/
/*****************************************************************/
pub fn timer_clear(n: usize,  elapsed: &mut [Duration; 64]) {
	elapsed[n] = Duration::ZERO;
}

/*****************************************************************/
/******            T  I  M  E  R  _  S  T  A  R  T          ******/
/*****************************************************************/
pub fn timer_start(n: usize,  start: &mut [Instant; 64]){
	start[n] = elapsed_time();
}

/*****************************************************************/
/******            T  I  M  E  R  _  S  T  O  P             ******/
/*****************************************************************/
pub fn timer_stop(n: usize, start: &mut [Instant; 64], elapsed: &mut [Duration; 64]){
	elapsed[n] = start[n].elapsed();
}

/*****************************************************************/
/******            T  I  M  E  R  _  R  E  A  D             ******/
/*****************************************************************/
pub fn timer_read(n: usize, elapsed: &mut [Duration; 64]) -> Duration{
	elapsed[n]
}
/* 
fn main() {
    
    let mut start: [Instant; 64] = [Instant::now(); 64];
    let mut elapsed: [Duration; 64] = [Duration::ZERO; 64];
    let mut x: i32 = 0;
    //let start1 = Instant::now();
    timer_clear(0, &mut elapsed);
    timer_start(0, &mut start);
    for i in 0..10000000 {
        x = i * 2;
    }
    println!("{}", x);
    //let duration = start1.elapsed();
    timer_stop(0, &mut start, &mut elapsed);

    println!("Tempo de execução: {:?}", timer_read(0, &mut elapsed));
}
*/
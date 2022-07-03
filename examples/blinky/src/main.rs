use blinky_implem::Lustre::System;
use blinky_implem::Blinky;

fn main() {
    let mut blinky_sys = Blinky::init((false,)).unwrap();
    blinky_sys.next((true,));
    loop {
        blinky_sys.next((false,));
        println!("{:?}", blinky_sys.output());

    }
}

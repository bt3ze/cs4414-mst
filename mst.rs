/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


extern mod extra;

use std::*;
use extra::arc;

struct Pixel{
    a: int
}

fn readImage( filename: &~str) -> ~ [ ~[Pixel]] {
    ~[~[]]
}

fn main(){
    let argv =  os::args();
    println(argv.to_str());
    let length = match from_str::<int>(argv[1]) {
        Some(x) => x,
        None() => 0
    };
    let width = match from_str::<int>(argv[2]) {
        Some(x) => x,
        None() => 0
    };
    
    let mut find_pixels: ~[ ~[Pixel] ] = readImage(&argv[0]);
    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[ ~[]];
    let mut len = 0;
    let mut wid = 0;
    loop {
        loop {
            find_arcs[len][wid] = arc::RWArc::new(find_pixels[len][wid]);
                wid+=1;
            if(wid == width) {
                break; 
            }
        }
        len+=1;
        if(len == length) {
            break; 
        }
    }
    
    let arcs = find_arcs; // now we have an immutable array that can be shared
    
}

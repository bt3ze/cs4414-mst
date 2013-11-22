/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


extern mod extra;

use std::{os, vec,int};
use extra::arc;

struct Pixel{
    a: int
}

fn readImage( filename:  &[~str]) -> ~ [ ~[Pixel]]{
    ~[~[]]
}

fn main(){
    let argv =  os::args();
    println(argv);
    let length = match std::int::from_string(argv[1]){
        Some(x) => x,
        None() => 0
    };
    let width =  match std::int::from_string(argv[2]){
        Some(x) => x,
        None() => 0
    };

    let mut find_pixels: ~[ ~[Pixel] ] = readImage(argv[0]);
    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[ ~[]];
    let mut len = 0;
    let mut wid = 0;
    loop {
        loop {
            find_arcs[len][wid] = arc::RWArc::new(find_pixels[len][wid]);
            wid ++;
            if(wid == width)
                break;
        }
        len ++;
        if(len == length)
            break;
    }

/*    for i = 0 to length {
        for j = 0 to width {
            find_arcs[i][j] = arc::RWArc::new(find_pixels[i][j];
        }
    } */
    let arcs = find_arcs; // now we have an immutable array that can be shared
    
}

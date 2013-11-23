/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


extern mod extra;

use std::{os, num,str} ;
use extra::arc;

struct Pixel{
    r: int,
    b: int,
    g: int
}

impl Pixel {
    fn new(red: int, blue: int, green: int) -> Pixel {
        Pixel{
            r: red,
            b: blue,
            g: green
        }
    }
}

struct Edge {
    source: Pixel,
    dest: Pixel,
    cost: float
}

impl Edge {
    fn new(s: Pixel, d: Pixel) -> Edge {
        Edge {
            source: s,
            dest: d,
            cost: edgeCost(s,d)
        }
    }
}

impl Ord for Edge {
    fn lt(&self, other: &Edge) -> bool {
        self.cost < other.cost
    }
}

fn edgeCost(a: Pixel, b: Pixel) -> float {
    num::sqrt( (
            a.r*a.r - b.r*b.r +
            a.g*a.g - b.g*b.g +
            a.b*a.b - b.b*b.b ) as float)
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
    
    let mut x= 0;
    let mut y= 0;
    
}

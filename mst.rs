/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


extern mod extra;
use std::{os, num,str} ;
use extra::{arc,priority_queue};

struct Pixel{
    r: int,
    b: int,
    g: int,

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
            a.r*a.r-b.r*b.r + a.g*a.g-b.g*b.g + a.b*a.b-b.b*b.b ) as float)
}

fn readImage( filename: &~str) -> ~ [ ~[Pixel]] {
    ~[~[]]
}

fn main(){
    let argv =  os::args();
    println(argv.to_str());
    let length = match from_str::<int>(argv[2]) {
        Some(x) => x,
        None() => 0
    };
    let width = match from_str::<int>(argv[3]) {
        Some(x) => x,
        None() => 0
    };
    
    let pixels: ~[ ~[Pixel] ] = readImage(&argv[1]);
    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[ ~[]];
    let mut len = 0;
    let mut wid = 0;
    loop {
        loop {
            find_arcs[len][wid] = arc::RWArc::new(pixels[len][wid]);
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
    let mut queue: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();

    loop { // iterate through items in the queue, which contains all of the edges/vertices
        // add (x,y+1), (x,y-1), (x-1,y), (x+1,y) to queue if not already visited or colored
        let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x,y-1)];
        for &coord in neighbors.iter() {
            // if neighbor at coord has not been visited or colored
            let (w,z) = coord;
            queue.push( Edge::new(pixels[x][y],pixels[w][z]) );
        }
    }

}

/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


/*
some documentation:
Pixel object is composed of r,g,b values, a color, and an x,y index that takes the place of its adjacency list
Point is just an (x,y) value to denote a pixel's location
Edge contains two points: source and dest, along with the edge cost between them

*/


extern mod extra;
use std::{os, num,hashmap,str} ;
use extra::{arc,priority_queue};

struct Pixel{
    r: int,
    b: int,
    g: int,
    color: int,
    x: int,
    y: int
}

impl Pixel {
    fn new(red: int, blue: int, green: int, x: int, y: int) -> Pixel {
        Pixel{
            r: red,
            b: blue,
            g: green,
            color: -1,
            x: x,
            y: y
        }
    }
}

struct Point {
    x: int,
    y: int
}

impl Point{
    fn new(x: int, y: int)-> Point {
        Point {
            x: x,
            y: y
        }
    }
}

struct Edge {
    source: Point,
    dest: Point,
    cost: float
}

impl Edge {
    fn new(s: Point, d: Point, cost: float) -> Edge {
        Edge { // these need to be references to pixels, not copies.
            // for standard prim, it's alright for now
            source: s,
            dest: d,
            cost: cost
        }
    }
}

impl Ord for Edge {
    fn lt(&self, other: &Edge) -> bool {
        self.cost < other.cost
    }
}

fn edgeCost(a: &Pixel, b: &Pixel) -> float {
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
                wid = 0;
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
    let mut visited: hashmap::HashMap<(int,int),bool> = hashmap::HashMap::new();

    loop { // iterate through items in the queue, which contains all of the edges/vertices
        
        let edge = queue.maybe_pop();
        match edge {
            Some(e) => {
                if e.dest.color < 0 {
                    // not in any tree
                    let coord = (e.dest.x,e.dest.y);
                    if *visited.get(&coord) == false { // use visited hashmap to figure out if we're at a new vertex
                        visited.insert(coord,true);
                        x = e.dest.x;
                        y = e.dest.y;
                        let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x,y-1)];
                        for &coord in neighbors.iter() {
                            let (w,z) = coord;
                            if w >=0 && w < width && z >=0 && z <= length { // bounds checking
                                // if neighbor at coord has not been or colored
                                if pixels[w][z].color < 0 { // then we have a new vertex
                                    queue.push( Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(&pixels[x][y],&pixels[w][z])));
                                }
                            }
                        }
                    }
                    
                    // use an arc to write the following
                    // e.dest.color = e.source.color;
                }
            }
            None => { break; } // this should really never execute till the end
        }
    }
    
    

}

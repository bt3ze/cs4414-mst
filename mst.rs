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
use std::{os, num,hashmap} ;
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

impl Clone for Pixel{
    fn clone(&self) -> Pixel {
        Pixel{
            r: self.r,
            b: self.b,
            g: self.g,
            color: self.color,
            x: self.x,
            y: self.y
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
        Edge {
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

    let pixels: ~[ ~[Pixel] ] = readImage(&argv[1]);

    // width and height should be replaced by just reading them from the file
    let width = match from_str::<int>(argv[2]) {
        Some(x) => x,
        None() => 0
    };
    let height = match from_str::<int>(argv[3]) {
        Some(x) => x,
        None() => 0
    };
    
    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[ ~[]];
    let mut heit = 0;
    let mut wid = 0;
    loop {
        loop {
            find_arcs[heit][wid] = arc::RWArc::new(pixels[heit][wid]);
                wid+=1;
            if(wid == width) {
                wid = 0;
                break; 
            }
        }
        heit+=1;
        if(heit == height) {
            break; 
        }
    }
    
    let arcs = find_arcs; // now we have an immutable array that can be shared
    
    let corners = [ (0,0),(0,width-1),(height-1,0),(height-1,width-1) ];
    
    for &corner in corners.iter() {
        let (a,b) = corner;
        let shared_arcs = arcs.clone();
        do spawn { // split into threads here
            
            let mut x=a;
            let mut y=b;
            /*
            weird compiler error:
            let corners = [ (0,0),(0,width-1),(height-1,0),(height-1,width-1) ];
            for &corner in corners.iter(){
            do spawn {
            let mut x = 0;
            let mut y = 0;
            (x,y) = corner
                 }
             }
             */


            let mut queue: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();
            let mut visited: hashmap::HashMap<(int,int),bool> = hashmap::HashMap::new();
            
            let mut newvisit: bool = true;
            
            loop { // iterate through items in the queue, which contains all of the edges/vertices     
                if(newvisit){
                    let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x,y-1)];
                    for &coord in neighbors.iter() {
                        let (w,z) = coord;
                        if w >=0 && w < width && z >=0 && z < height { // bounds checking
                            // if neighbor at coord has not been or colored
                            let mut newvertex = false;
                            let mut uncolored = false;
                            do shared_arcs[w][z].read |dest| {
                                if dest.color < 0 {
                                    uncolored = true; }
                            }
                            if uncolored {
                                do shared_arcs[w][z].write |dest| {
                                    if dest.color < 0 {
                                        do shared_arcs[x][y].read |src| {
                                            dest.color = src.color;
                                        }
                                        newvertex = true;
                                    }
                                }
                            }
                            if newvertex { // then we have a new vertex
                                do shared_arcs[x][y].read |src| {
                                    do shared_arcs[w][z].read |dest| {
                                        queue.push( Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(src,dest)));
                                    }
                                }
                            }
                        }
                    }
                }
                
                let edge = queue.maybe_pop();
                match edge {
                    Some(e) => {
                        // not in any tree
                        let coord = (e.dest.x,e.dest.y);
                        if *visited.get(&coord) == false { // use visited hashmap to figure out if we're at a new vertex. returns false if it can't return a vertex?
                            visited.insert(coord,true);
                            x = e.dest.x;
                            y = e.dest.y;
                            newvisit = true;
                        } else {
                            newvisit = false;
                        }
                        None => { break; } // this should execute at the end when there are no more unvisited or uncolored nodes for a thread
                    }
                }
            }
        }
    }
}

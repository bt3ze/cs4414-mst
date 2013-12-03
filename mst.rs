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
use std::{os, num, hashmap} ;
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
        self.cost > other.cost
    }
}

fn edgeCost(a: &Pixel, b: &Pixel) -> float {
    num::sqrt( (
            (a.r-b.r)*(a.r-b.r) + (a.g-b.g)*(a.g-b.g) + (a.b-b.b)*(a.b-b.b) ) as float)
}



fn readImage( filename: &~str) -> ~ [ ~[Pixel]] {
    let mut retval: ~[~[Pixel]] = ~[];

    let file: &str = filename.to_owned();
//    let file2: ~str = file.to_owned();
    let split_filename: ~[&str] = file.split_iter('.').collect();
    let file_data: &str = split_filename[0] + ".txt";

    println(fmt!("The image name is [%?]", file));
    println(fmt!("The image data will be stored at [%?]", file_data));

    //read in the .txt associated with the file title
    let path = &PosixPath(file_data);

    match std::io::file_reader(path) {
	Ok(reader) => { 
	    //run the java program to process the image
/*	    let program: &str = "java ImageReader";
	    let argv: ~[~str] = ~[file2];
	    let mut prog = std::run::Process::new(program, argv, 
		std::run::ProcessOptions {
			env: None,
			dir: None,
			in_fd: None,
			out_fd: None,
			err_fd: None
		}
	    );
	    prog.finish();
*/
	    //parse out the array dimensions
    	    let dimensions_str: &str = reader.read_line();
    	    println(fmt!("dimensons: %s",dimensions_str));
	    let mut h: int = 0;
	    let mut w: int = 0;
	    match dimensions_str[0] as char {
		'H' => {
		    let split_dim: ~[&str] = dimensions_str.split_iter('W').collect();
		    h = from_str(split_dim[0].slice(1,split_dim[0].len())).unwrap();
		    w = from_str(split_dim[1]).unwrap();
		    println(fmt!("Dimensions passed: H %? W %?",h,w));
		    //let retval: ~[ ~[Pixel, ..w], ..h];    
		}
		_ => {println("Dimensions corrupted! Abort");}
	    }

	    //fill in the retVal array with pixel data
            //let mut rowfirst = false;
	    for row in range(0,h){
//                if(!rowfirst) {
                retval.push(~[]);
                //rowfirst = true; }
		for col in range(0,w){
	    	    if !reader.eof() {
			let line: &str = reader.read_line();
			//println(line);
			let rpos:uint = match line.find('R') { Some(n)=>n,None=>line.len() };
			let gpos:uint = match line.find('G') { Some(n)=>n,None=>line.len() };
			let bpos:uint = match line.find('B') { Some(n)=>n,None=>line.len() };
			
			if rpos == line.len() || gpos == line.len() || bpos == line.len() {
			    println("Pixel data corrupted! Abort");
			}

			let R:int = from_str(line.slice(rpos+1,gpos)).unwrap();
			let G:int = from_str(line.slice(gpos+1,bpos)).unwrap();
			let B:int = from_str(line.slice(bpos+1,line.len())).unwrap();

			//println(fmt!("R %? G %? B %?",R,G,B));

			//fn new(red: int, blue: int, green: int, x: int, y: int) -> Pixel {
			retval[row].push(Pixel::new(R,G,B,col,row));
			

		    }
		    else {
			println("Data file corrupted! Abort");
		    }
		}
	    }
	},
	Err(err)   => { fail!(err) }
    }

    retval
}

fn main(){
    let argv =  os::args();
    println(argv.to_str());

    let pixels: ~[ ~[Pixel] ] = readImage(&argv[1]);
    let height = pixels.len() as int;
    let width = pixels[0].len() as int;

    println(fmt!("Dimensions received: H %? W %?",height,width));

    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[];
    for h in range(0,height){
	find_arcs.push(~[]);
        for w in range(0,width){
 //           println(fmt!("pix %?",pixels[h][w]));
	    find_arcs[h].push(arc::RWArc::new(pixels[h][w]));
	}
    }
    
    let arcs = find_arcs; // now we have an immutable array that can be shared
    
    let corners = [ (0,0),(width-1,0),(0,height-1),(width-1,height-1) ];
    
    let mut color_index = 0;
    for &corner in corners.iter(){
        let (a,b) = corner;
        do arcs[b][a].write |pix| {
            pix.color = color_index;
        }
        color_index+=1;
    }
    
    for &corner in corners.iter() {
        let (a,b) = corner;
        let shared_arcs = arcs.clone();
        do spawn { // split into threads here
            let c = a;
            let d = b;
            let mut x=a;
            let mut y=b;

            let mut queue: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();
            let mut visited: hashmap::HashMap<(int,int),bool> = hashmap::HashMap::new();            
            let mut newvisit: bool = true;
            let mut boundaries: ~[Edge] = ~[];
            let mut color: int = -1;
            do shared_arcs[b][a].read |pix| {
                color = pix.color;
            }
            

            println(fmt!("start at (%i,%i): color = %i",c,d,color));

            loop { // iterate through items in the queue, which contains all of the edges/vertices     
                if(newvisit){
//                   println(fmt!("(%i,%i) : (%i,%i)",c,d,x,y));
                    let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x-1,y)];
                    for &coord in neighbors.iter() {
                        let (w,z) = coord;
                        if w >=0 && w < width && z >=0 && z < height { // bounds checking
                            // if neighbor at coord has not been colored
                            let mut newvertex = false;
                            let mut colored = false;
                            do shared_arcs[z][w].read |dest| {
                                if dest.color >= 0 {
                                    colored = true;
                                }                         
                            }
                            if !colored {
                                do shared_arcs[z][w].write |dest| {
                                    if dest.color < 0 {
                                        dest.color = color;
                                        newvertex = true;
                                    } else {
                                        // edge has crossed a cut during a race since we last checked if it was uncolored
                                        colored = true;
                                    }
                                }
                            }
                            if colored { // this is an else to the above if
                                // we have found an edge that crosses a cut!
                                // here, somehow collect the edges that cross our cut
                                do shared_arcs[y][x].read |src| {
                                    do shared_arcs[z][w].read |dest| {
                                        if dest.color != color {
                                            boundaries.push( Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(src,dest)));
                                            println(fmt!("%?\n-->%?",src,dest));
 
                                        }
                                    }
                                }  
                            }
                            if newvertex { // then we have a new vertex
                                do shared_arcs[y][x].read |src| {
                                    do shared_arcs[z][w].read |dest| {
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
                        println(fmt!("(%i,%i) pop (%i,%i)-%f-(%i,%i)",c,d,e.source.x,e.source.y,e.cost,e.dest.x,e.dest.y));
                        // not in any tree
                        let coord = (e.dest.x,e.dest.y);
//                        println(fmt!("before runtime error? %?",coord));
                        if !visited.contains_key(&coord){
 // use visited hashmap to figure out if we're at a new vertex.
                            visited.insert(coord,true);
                            x = e.dest.x;
                            y = e.dest.y;
                            newvisit = true;
//                            println(fmt!("%? %f",coord,e.cost));
                        } else {
                            newvisit = false;
                        }
                    },
                    None => { break; } // this should execute at the end when there are no more unvisited or uncolored nodes for a thread
                }   
            }
            
            for e in boundaries.iter() {
                println(fmt!("(%i,%i:%i) boundary (%i,%i)-%f-(%i,%i)",c,d,color,e.source.x,e.source.y,e.cost,e.dest.x,e.dest.y));
            }
        }
    }
    
    for h in range(0,height){
        for w in range(0,width){
            do arcs[h][w].read |pix| {
                println(fmt!("color(%i,%i) %i",w,h,pix.color));
	    }
        }
    }
}

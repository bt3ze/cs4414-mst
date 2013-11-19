extern mod std;
extern mod extra;

struct ChildNode<'self> {
    pixel: Pixel,
    parent: &'self ParentNode 
}

impl<'self> ChildNode {
     fn new(pixel: Pixel) -> ChildNode {
     	let edgelist = ~[];
	ChildNode {
		  pixel: pixel,
		  parent: ParentNode {
		  	  edges: edgelist
		 }		  
	}
     }       
}

struct ParentNode<'self> {
    edges: &'self [Edge]
}

struct Point {
       x: int,
       y: int
}

//the source and dest 
struct Edge<'self> {
    source: Point,
    dest: Point,
    weight: int
}

impl<'self> Edge {
    fn new<'e>(source: Point, dest: &'e Point)-> &'e Edge {
        Edge {
            source: source,
            dest: dest,
            weight: 0
        }
    }
}


struct Pixel {
    r: int,
    g: int,
    b: int
}

fn main() {
 //   let pixels: ~[Pixel] = processImage();
 return 1;
}

extern mod std;
extern mod extra;

struct ChildNode {
    pixel: Pixel,
    parent: ParentNode
}

impl ChildNode {

}

struct Edge {
    source: &ChildNode,
    dest: &ChildNode,
    weight: int
}

impl Edge {
    fn new(source: @ChildNode, dest: @ChildNode)-> Edge {
        Edge {
            source: source,
            dest: dest,
            weight: 0
        }
    }
}

struct ParentNode {
    edges: @[Edge]
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

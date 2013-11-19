extern mod std;
extern mod extra;

struct ChildNode {
    pixel: Pixel;
    parent: ParentNode;
}

impl ChildNode {

}

struct Edge {
    source: &ChildNode;
    dest: &ChildNode;
}

struct ParentNode {
    edges: ~[Edge];
}

struct Pixel {
    r: int;
    g: int;
    b: int;
}

fn main() {
    let pixels: ~[Pixel] = processImage();

}

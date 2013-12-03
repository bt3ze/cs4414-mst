all: prim img boruvka

mst: img prim

img: ImageReader.class ImageReader.java
	java ImageReader step_gradient.jpg

prim: mst.rs
	rustc mst.rs

boruvka: boruvka.rs
	rustc boruvka.rs

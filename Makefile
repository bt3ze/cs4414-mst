all: prim img boruvka

mst: img sample prim

img: ImageReader.class ImageReader.java
	java ImageReader step_gradient.jpg

sample: ImageReader.class ImageReader.java
	java ImageReader sample.jpg

prim: mst.rs
	rustc mst.rs

boruvka: boruvka.rs
	rustc boruvka.rs

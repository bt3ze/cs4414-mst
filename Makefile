all: mst images

mst: prim

images: gradient sample corners dancers

gradient: ImageReader.class ImageReader.java
	java ImageReader step_gradient.jpg

sample: ImageReader.class ImageReader.java
	java ImageReader sample.jpg

corners: ImageReader.class ImageReader.java
	java ImageReader small-corners.jpg

dancers: bluedancers purpledancers

bluedancers: ImageReader.class ImageReader.java
	java ImageReader bluedancers.jpg

purpledancers: ImageReader.class ImageReader.java
	java ImageReader purpledancers.jpg

prim: mst.rs
	rustc mst.rs

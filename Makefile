all: prim boruvka

prim: mst.rs
	rustc mst.rs

boruvka: boruvka.rs
	rustc boruvka.rs

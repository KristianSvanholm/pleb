{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
    buildInputs = [
	python312 
	libgcc # C/C++
	rustup # Rust
	zulu23 # Java
	graalvm-ce # Java Graal
	ghc # Haskell
	dart 
	erlang 
	go
	fsharp
	gfortran # Fortran
	gnat # Ada
	jruby
	ruby
	nodejs_23 # JavaScript
	julia
	sbcl # Lisp
	lua
	ocaml
	php
	fpc # Pascal
	perl
	racket
	ruby
	swift
	gnu-smalltalk # Smalltalk
	mono # Csharp
	typescript
    ];
}
  

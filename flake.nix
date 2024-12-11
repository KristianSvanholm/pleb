{
    description = "Dependencies";

    inputs = {
	nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	flake-utils.url = "github:numtide/flake-utils";
	hhvm.url = "github:facebook/hhvm";
    };

    outputs = { self, nixpkgs, flake-utils, hhvm }@inputs: 
	flake-utils.lib.eachDefaultSystem
	    (system:
		let 
		    pkgs = import nixpkgs {
			inherit system;
			#overlays = [hhvm];
		    };
		in
		with pkgs;
		{
		    devShells.default = mkShell {
			buildInputs = [
			    python312 
			    libgcc # C/C++
			    rustc # Rust
			    jdk # Java
			    ghc # Haskell
			    dart 
			    erlang 
			    go
			    fsharp
			    gfortran # Fortran [ifx would not work for apple]
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
			    dotnet-sdk
			    #hhvm # Hack. Cant get this to work yet.
			    #gnu-smalltalk # Smalltalk
			    #graalvm-ce # Java Graal
			];
		    };
		}
	    );
}

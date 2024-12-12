{
    description = "Dependencies";

    inputs = {
	nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	flake-utils.url = "github:numtide/flake-utils";
	old-ocaml.url = "github:nixos/nixpkgs/54c1e44240d8a527a8f4892608c4bce5440c3ecb";
	#hhvm.url = "github:nixos/nixpkgs/db8a4a4ef5644652bba98243805323eb7bf10404";
    };

    outputs = { self, nixpkgs, flake-utils, ... }@inputs: 
	flake-utils.lib.eachDefaultSystem
	    (system:
		let 
		    pkgs = import nixpkgs {
			inherit system;
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
			    llvmPackages_15.libllvm # Haskell
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
			    php
			    fpc # Pascal
			    perl
			    racket
			    ruby
			    swift
			    dotnet-sdk
			    typescript
			    inputs.old-ocaml.legacyPackages.${system}.ocaml # Ocaml 4.14.1
			    #inputs.hhvm.legacyPackages.${system}.hhvm
			    #hhvm # Hack. Cant get this to work yet.
			    #gnu-smalltalk # Smalltalk
			    #graalvm-ce # Java Graal

			    # Pidigits
			    gmp
			    python312Packages.gmpy2

			    #libApr
			    apr

			    #pcre
			    pcre

			    #khash
			    htslib
			];
		    };
		}
	    );
}

#!/bin/bash

echo "Generating input for k-nucleotide benchmark"
python Python/fasta/fasta.python3-3.py 25000000 > knucleotide-input25000000.txt

echo "Generating input for reverse-complement benchmark"
python Python/fasta/fasta.python3-3.py 25000000 > revcomp-input25000000.txt


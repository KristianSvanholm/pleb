# Thesis
Cross platform process energy benchmarking tool.

## Mac OS
Utilizes IOReport, an undocumented Apple API.

I performed open source surgery on [vladkens/macmon](https://github.com/vladkens/macmon), to pick out the pieces I needed. Many thanks!

Followed [this](https://medium.com/@vladkens/how-to-get-macos-power-metrics-with-rust-d42b0ad53967) article to achieve this. Again, by [Vladkens](https://github.com/vladkens).

## Linux 
Utilizes Powercap to access Intel RAPL. 

Might add support for AMD RAPL in the future :)


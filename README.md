Sådan kører du programmet (erstat <width> og <height> med de ønskede værdier):

sekventiel version
cargo run --release -- <width> <height>

parallel version
cargo run --release -- --parallel <width> <height>


hyperfine benchmark test
NB: Sørg for at have hyperfine installeret og at du er i det rigtige mappe, hvor mandelbrot.exe ligger.

sekventiel version
hyperfine '.\target\release\mandelbrot.exe --benchmark <width> <height>'

parallel version
hyperfine '.\target\release\mandelbrot.exe --benchmark --parallel <width> <height>'
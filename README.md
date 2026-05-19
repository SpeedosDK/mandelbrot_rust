# Mandelbrot Generator

Genererer et billede af Mandelbrot-sættet i Rust.

> Erstat `WIDTH` og `HEIGHT` med de ønskede pixelværdier, f.eks. `1920 1080`.

---

## Kør programmet

### Sekventiel version
```bash
cargo run --release -- WIDTH HEIGHT
```

### Parallel version
```bash
cargo run --release -- --parallel WIDTH HEIGHT
```

### Forbedret parallel version (anbefalet)
Den forbedrede parallelle version paralleliserer over **rækker** i stedet for individuelle pixels.
Hver række beregnes i en lokal buffer og merges til sidst — dette giver bedre cache-udnyttelse og undgår overhead fra `par_bridge()`.

```bash
cargo run --release -- --parallel2 WIDTH HEIGHT
```

> Denne version er typisk hurtigere end `--parallel` på grund af mere effektiv arbejdsfordeling med Rayon.

---

## Benchmark med Hyperfine

> **Krav:** [hyperfine] skal være installeret, og du skal befinde dig i projektmappen.

### Sekventiel version
```bash
hyperfine '.\target\release\mandelbrot.exe --benchmark WIDTH HEIGHT'
```

### Parallel version
```bash
hyperfine '.\target\release\mandelbrot.exe --benchmark --parallel WIDTH HEIGHT'
```

### Forbedret parallel version
```bash
hyperfine '.\target\release\mandelbrot.exe --benchmark --parallel2 WIDTH HEIGHT'
```


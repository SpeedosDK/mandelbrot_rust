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

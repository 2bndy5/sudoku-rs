use criterion::{Criterion, criterion_group, criterion_main};
use sudoku_gen::{Board, BoardSize, RegionKind};

fn gen_solve(size: BoardSize) {
    let mut board = Board::new(&size, crate::RegionKind::Regular);
    assert!(board.generate());
}

fn gen_solve_4x4() {
    gen_solve(BoardSize::X4);
}

fn gen_solve_6x6() {
    gen_solve(BoardSize::X6);
}

fn gen_solve_9x9() {
    gen_solve(BoardSize::X9);
}

fn gen_solve_12x12() {
    gen_solve(BoardSize::X12);
}

// fn gen_solve_16x16() {
//     gen_solve(BoardSize::X16);
// }

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gen4x4", |b| b.iter(|| gen_solve_4x4()));
    c.bench_function("gen6x6", |b| b.iter(|| gen_solve_6x6()));
    c.bench_function("gen9x9", |b| b.iter(|| gen_solve_9x9()));
    c.bench_function("gen12x12", |b| b.iter(|| gen_solve_12x12()));
    // c.bench_function("gen16x16", |b| b.iter(|| gen_solve_16x16()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

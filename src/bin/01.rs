use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    let f = std::io::BufReader::new(std::fs::File::open("data/input1.txt")?);
    let mut heap = std::collections::BinaryHeap::new();
    let mut cur = 0;
    for l in f.lines().chain([Ok("".to_string())]) {
        let l = l?;
        if l.is_empty() {
            heap.push(cur);
            cur = 0;
            continue;
        }
        cur += l.parse::<i32>()?;
    }
    heap.peek().map(|i| println!("Part1: {}", i));
    println!("Part2: {}", (0..3).filter_map(|_| heap.pop()).sum::<i32>());
    Ok(())
}

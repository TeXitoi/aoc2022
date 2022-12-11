use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    let f = std::io::BufReader::new(std::fs::File::open("data/input1.txt")?);
    let mut heap = std::collections::BinaryHeap::new();
    let mut cur = 0;
    for l in f.lines() {
        let l = l?;
        if l.is_empty() {
            heap.push(cur);
            cur = 0;
            continue;
        }
        cur += l.parse::<i32>()?;
    }
    if cur != 0 {
        heap.push(cur);
    }
    let mut tot = 0;
    for m in (0..3).filter_map(|_| heap.pop()) {
        println!("{}", m);
        tot += m;
    }
    println!("sum3: {}", tot);
    Ok(())
}

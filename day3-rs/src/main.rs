use std::collections::HashSet;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Prio(u8);

impl TryFrom<u8> for Prio {
    type Error = ();

    fn try_from(value: u8) -> Result<Prio, ()> {
        match value {
            b'a'..=b'z' => Ok(Prio(value - b'a' + 1)),
            b'A'..=b'Z' => Ok(Prio(value - b'A' + 27)),
            _ => Err(()),
        }
    }
}

impl Prio {
    fn prio_value(self) -> i32 {
        match self {
            Prio(v) => v as i32,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day3.txt").expect("reading input file");
    let lines = input.lines();
    let rucksacks = lines.map(|line| {
        let (fst, snd) = line.split_at(line.len() / 2);
        let (fst_hash, snd_hash) = (
            fst.bytes().collect::<HashSet<_>>(),
            snd.bytes().collect::<HashSet<_>>(),
        );
        Prio::try_from(
            fst_hash
                .intersection(&snd_hash)
                .cloned()
                .collect::<HashSet<_>>()
                .into_iter()
                .nth(0)
                .unwrap(),
        )
        .unwrap()
        .prio_value()
    });
    // let res = Vec::from_iter(rucksacks);
    let res: i32 = rucksacks.sum();
    println!("{res:?}");
}

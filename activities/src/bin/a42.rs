// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct ScoreMultiplier {
    current: usize,
    step: usize,
}

impl ScoreMultiplier {
    fn new() -> Self {
        Self {
            current: 0,
            step: 1,
        }
    }

    fn powerup(&mut self, amount: usize) {
        self.step += amount;
    }
}

impl Iterator for ScoreMultiplier {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.step;
        Some(self.current)
    }
}

fn main() {
    let mut multiplier = ScoreMultiplier::new();

    // Default step of 1: produces 1, 2, 3
    println!("Multiplier: {:?}", multiplier.next()); // 1
    println!("Multiplier: {:?}", multiplier.next()); // 2
    println!("Multiplier: {:?}", multiplier.next()); // 3

    // Apply a +1 powerup, step becomes 2: produces 5, 7, 9
    multiplier.powerup(1);
    println!("-- powerup obtained! --");
    println!("Multiplier: {:?}", multiplier.next()); // 5
    println!("Multiplier: {:?}", multiplier.next()); // 7
    println!("Multiplier: {:?}", multiplier.next()); // 9

    // Apply another +1 powerup, step becomes 3: produces 12, 15, 18
    multiplier.powerup(1);
    println!("-- powerup obtained! --");
    println!("Multiplier: {:?}", multiplier.next()); // 12
    println!("Multiplier: {:?}", multiplier.next()); // 15
    println!("Multiplier: {:?}", multiplier.next()); // 18
}

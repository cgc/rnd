use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;

use counter::Counter;
use json::object;
use meansd::MeanSD;
use rand::SeedableRng;
use statrs::distribution::{Bernoulli, Categorical};
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use itertools::Itertools;
use clap::{Parser,command,arg};

type Rng = StdRng;

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialOrd, Ord)]
pub enum Card {
    Land,
    Other,
    Summon { cost: usize, power: usize, toughness: usize },
}

fn print_deck(deck: &Counter<Card>) {
    for (key, ct) in deck.iter().sorted() {
        if *ct == 0 {
            continue
        }
        println!("{ct} {key:?}");
    }
}

pub fn sample_card(deck: &Counter<Card>, r: &mut Rng) -> Card {
    let mut cards = vec![];
    let mut mass = vec![];
    // NOTE: Need to sort to ensure consistent ordering.
    for (key, count) in deck.iter().sorted() {
        cards.push(key);
        mass.push(*count as f64);
    }
    let n = Categorical::new(&mass).unwrap();
    let s: usize = n.sample(r);
    return *cards[s];
}

pub struct FieldStats {
    lands: usize,
    power: usize,
    toughness: usize,
    summons: usize,
    hand: usize,
}

pub struct Field {
    pub deck: Counter<Card>,
    pub hand: Counter<Card>,
    pub tapped: Counter<Card>,
    pub untapped: Counter<Card>,
}

impl Field {
    pub fn new(deck: Counter<Card>) -> Field {
        Field {
            hand: Field::empty_counter_like(&deck),
            tapped: Field::empty_counter_like(&deck),
            untapped: Field::empty_counter_like(&deck),
            deck,
        }
    }

    fn empty_counter_like(counter: &Counter<Card>) -> Counter<Card> {
        // Intention here is to pre-allocate as much as possible.
        let mut ct = Counter::with_capacity(counter.capacity());
        for key in counter.keys() {
            ct[key] = 0;
        }
        ct
    }

    pub fn move_card(src: &mut Counter<Card>, dest: &mut Counter<Card>, c: &Card) {
        src[&c] -= 1;
        dest[&c] += 1;
    }

    pub fn draw_card(&mut self, r: &mut Rng) -> Card {
        let c = sample_card(&self.deck, r);
        Field::move_card(&mut self.deck, &mut self.hand, &c);
        c
    }

    pub fn init(&mut self, r: &mut Rng) {
        for _ in 0..7 {
            self.draw_card(r);
        }
    }

    pub fn begin(&mut self, r: &mut Rng) {
        self.draw_card(r);
        self.untap();
    }

    pub fn untap(&mut self) {
        self.untapped += self.tapped.clone();
        self.tapped.clear();
    }

    pub fn played_stats(&self) -> FieldStats {
        let played = self.untapped.clone() + self.tapped.clone();
        let mut lands = 0;
        let mut power = 0;
        let mut toughness = 0;
        let mut summons = 0;
        for (key, count) in played.iter() {
            match key {
                Card::Land => lands += count,
                Card::Summon { cost: _, power: p, toughness: t} => {
                    summons += count;
                    power += p * count;
                    toughness += t * count;
                },
                Card::Other => {},
            }
        }
        let hand = self.hand.total();
        FieldStats { lands, power, toughness, summons, hand }
    }

    pub fn end(&mut self) {}

    pub fn play(&mut self, card: Card) -> Option<()> {
        if self.hand[&card] == 0 {
            return None
        }
        match card {
            Card::Land => {
                // Move card
                self.hand[&card] -= 1;
                self.untapped[&card] += 1;
            },
            Card::Summon { cost, power: _, toughness: _ } => {
                if self.untapped[&Card::Land] < cost {
                    return None
                }
                // Move card
                self.hand[&card] -= 1;
                self.tapped[&card] += 1;
                // Pay cost
                self.untapped[&Card::Land] -= cost;
                self.tapped[&Card::Land] += cost;
            }
            Card::Other => unreachable!("should not play Card::Other"),
        }
        Some(())
    }

    pub fn play_while_possible(&mut self, card: Card) {
        let mut res = self.play(card);
        while res.is_some() {
            res = self.play(card);
        }
    }

    pub fn show(&self) {
        println!("hand:");
        print_deck(&self.hand);
        println!("tapped:");
        print_deck(&self.tapped);
        println!("untapped:");
        print_deck(&self.untapped);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counter_from(cards: &[(Card, usize)]) -> Counter<Card> {
        let mut ct = Counter::new();
        for (card, count) in cards {
            ct[card] += count;
        }
        ct
    }
    fn assert_eq_ct(a: &Counter<Card>, b: &Counter<Card>) {
        for key in (a.clone() | b.clone()).keys() {
            assert_eq!(a[key], b[key], "Testing for {key:?}");
        }
    }

    #[test]
    fn test_field() {
        let mut deck: Counter<Card> = Counter::new();
        let c1 = Card::Summon { cost: 1, power: 1, toughness: 1 };
        let c2 = Card::Summon { cost: 2, power: 2, toughness: 2 };
        let l = Card::Land;
        deck[&l] += 40;
        deck[&c1] += 20;
        assert_eq!(deck.total::<usize>(), 60);
        let mut field = Field::new(deck);
        let mut r = StdRng::seed_from_u64(3667);

        field.hand[&l] += 3;
        field.hand[&c1] += 2;
        field.hand[&c2] += 2;

        assert_eq!(field.hand.total::<usize>(), 7);
        field.begin(&mut r);
        assert_eq!(field.tapped.total::<usize>(), 0);
        assert_eq!(field.untapped.total::<usize>(), 0);
        field.play(l);
        assert_eq!(field.tapped.total::<usize>(), 0);
        assert_eq_ct(&field.untapped, &counter_from(&[(l, 1)]));
        field.play_while_possible(c1);
        assert_eq_ct(&field.tapped, &counter_from(&[(l, 1), (c1, 1)]));
        assert_eq!(field.untapped.total::<usize>(), 0);

        assert_eq!(field.hand.total::<usize>(), 6);
        field.begin(&mut r);
        assert_eq!(field.tapped.total::<usize>(), 0);
        assert_eq_ct(&field.untapped, &counter_from(&[(l, 1), (c1, 1)]));
        field.play(l);
        assert_eq!(field.tapped.total::<usize>(), 0);
        assert_eq_ct(&field.untapped, &counter_from(&[(l, 2), (c1, 1)]));
        field.play_while_possible(c2);
        field.play_while_possible(c1);
        assert_eq_ct(&field.tapped, &counter_from(&[(l, 2), (c2, 1)]));
        assert_eq_ct(&field.untapped, &counter_from(&[(c1, 1), (l, 0)]));

        assert_eq!(field.played_stats().power, 3);

        assert_eq!(field.hand.total::<usize>(), 5);
        field.begin(&mut r);
        field.play(l);
        field.play_while_possible(c2);
        field.play_while_possible(c1);
        assert_eq_ct(&field.tapped, &counter_from(&[(l, 3), (c1, 1), (c2, 1)]));
        assert_eq_ct(&field.untapped, &counter_from(&[(l, 0), (c1, 1), (c2, 1)]));

        assert_eq!(field.played_stats().power, 6);

        assert_eq!(field.hand.total::<usize>(), 3);
    }

    #[test]
    fn test_cumulative_sum() {
        assert_eq!(cumulative_sum(&vec![1, 4, 2, 5]), vec![1, 5, 7, 12])
    }

    #[test]
    fn test_smallest_k() {
        let mut s = SmallestK::new(3);
        s.push(3);
        s.push(-1);
        assert_eq!(s.max_heap.iter().cloned().sorted().collect::<Vec<i32>>(), vec![-1, 3]);

        s.push(2);
        assert_eq!(s.max_heap.iter().cloned().sorted().collect::<Vec<i32>>(), vec![-1, 2, 3]);

        // ejects non-smallest, maintains size
        s.push(1);
        assert_eq!(s.max_heap.iter().cloned().sorted().collect::<Vec<i32>>(), vec![-1, 1, 2]);

        // ignores if not smaller, maintains size
        s.push(10);
        assert_eq!(s.max_heap.iter().cloned().sorted().collect::<Vec<i32>>(), vec![-1, 1, 2]);
    }
}


fn sim(deck: Counter<Card>, r: &mut Rng, turns: usize, log: bool) -> (Field, Vec<FieldStats>) {
    let mut field = Field::new(deck);
    let mut stats = vec![];
    field.init(r);
    for t in 0..turns {
        field.begin(r);
        field.play(Card::Land);
        for n in (0..10).rev() {
            let c = Card::Summon { cost: n, power: n, toughness: n };
            field.play_while_possible(c);
        }
        field.end();
        stats.push(field.played_stats());
        if log {
            println!("End of turn {t}");
            field.show();
            println!("");
        }
    }
    (field, stats)
}

fn update_meansds(meansds: &mut Vec<MeanSD>, value: &Vec<usize>) {
    for (meansd, value) in meansds.iter_mut().zip(value.iter()) {
        meansd.update(*value as f64)
    }
}

fn sims(deck: Counter<Card>, r: &mut Rng, trials: usize, turns: usize) -> HashMap<&'static str, Vec<MeanSD>> {
    let mut rv = HashMap::new();
    for key in ["lands", "power", "cumu_lands", "cumu_power", "hand"] {
        rv.insert(key, vec![MeanSD::default(); turns]);
    }

    for _ in 0..trials {
        let (_f, stats) = sim(deck.clone(), r, turns, false);

        let lands = stats.iter().map(|s| s.lands).collect();
        update_meansds(rv.get_mut("lands").unwrap(), &lands);
        let cumu_lands = cumulative_sum(&lands);
        update_meansds(rv.get_mut("cumu_lands").unwrap(), &cumu_lands);

        let power = stats.iter().map(|s| s.power).collect();
        update_meansds(rv.get_mut("power").unwrap(), &power);
        let cumu_power = cumulative_sum(&power);
        update_meansds(rv.get_mut("cumu_power").unwrap(), &cumu_power);

        update_meansds(rv.get_mut("hand").unwrap(), &stats.iter().map(|s| s.hand).collect());
    }
    rv
}

// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
pub fn mean(data: &[usize]) -> f64 {
    let sum = data.iter().sum::<usize>() as f64;
    return sum / data.len() as f64;
}

pub fn std_deviation(data: &[usize]) -> f64 {
    let data_mean = mean(data);
    let variance = data.iter().map(|value| {
        let diff = data_mean - (*value as f64);
        diff * diff
    }).sum::<f64>() / data.len() as f64 as f64;
    variance.sqrt()
}

pub fn std_err_mean(data: &[usize]) -> f64 {
    std_deviation(data) / (data.len() as f64).sqrt()
}

pub fn cumulative_sum(data: &Vec<usize>) -> Vec<usize> {
    data.iter().scan(0, |acc, &el| {
        *acc = *acc + el;
        Some(*acc)
    }).collect()
}

pub fn make_deck(name: &str, n_lands: usize) -> Counter<Card> {
    let c1 = Card::Summon { cost: 1, power: 1, toughness: 1 };
    let c2 = Card::Summon { cost: 2, power: 2, toughness: 2 };
    let c3 = Card::Summon { cost: 3, power: 3, toughness: 3 };
    let c4 = Card::Summon { cost: 4, power: 4, toughness: 4 };
    let c5 = Card::Summon { cost: 5, power: 5, toughness: 5 };
    let c6 = Card::Summon { cost: 6, power: 6, toughness: 6 };

    let mut deck: Counter<Card> = Counter::new();
    deck[&Card::Land] += n_lands;

    match name {
        "mono" => {
            // sort of based on a mono-red deck
            // originally, it's 24 lands, 24 creatures (12x1, 8x2, 4x3), rest spells
            // https://mtga.untapped.gg/meta/decks/510/mono-red-aggro/AAQAAQABiKIyAaXULArswgHGkAHbEt7tKbTvBBgIELUDjwcBD-IJAA?tab=overview
            let n_creatures = 60 - n_lands;
            deck[&c1] += n_creatures / 2;
            deck[&c2] += n_creatures / 3;
            deck[&c3] += n_creatures / 6;
            assert!(deck.total::<usize>() <= 60);
            deck[&Card::Other] += 60 - deck.total::<usize>();
        },
        "mono2" => {
            // sort of based on this, took middle of their ranges
            // https://magic.wizards.com/en/news/feature/how-build-mana-curve-2017-05-18
            // 1 + 5 + 4 + 3 + 2 + 1 = 16
            let n_creatures = 60 - n_lands;
            deck[&c1] += (n_creatures as f64 * 1./16.).floor() as usize;
            deck[&c2] += (n_creatures as f64 * 5./16.).floor() as usize;
            deck[&c3] += (n_creatures as f64 * 4./16.).floor() as usize;
            deck[&c4] += (n_creatures as f64 * 3./16.).floor() as usize;
            deck[&c5] += (n_creatures as f64 * 2./16.).floor() as usize;
            deck[&c6] += (n_creatures as f64 * 1./16.).floor() as usize;
            assert!(deck.total::<usize>() <= 60);
            deck[&Card::Other] += 60 - deck.total::<usize>();
        },
        "pow1" => {
            deck[&c1] += 60 - deck.total::<usize>();
        },
        _ => unreachable!(),
    }
    deck
}

fn eval(deck: &str, trials: usize, turns: usize) {
    let mut r = StdRng::seed_from_u64(3663);

    let mut n_lands_ = vec![];
    let mut turn = vec![];
    let mut lands = vec![];
    let mut lands_sem = vec![];
    let mut power = vec![];
    let mut power_sem = vec![];
    let mut hand = vec![];
    let mut hand_sem = vec![];

    let mut fin_n_lands_ = vec![];
    let mut fin_name = vec![];
    let mut fin_value = vec![];
    let mut fin_value_sem = vec![];
    let mut fin_cumulative = vec![];

    for n_lands in 10..=30 {
        println!("with # lands {n_lands}");
        let deck = make_deck(deck, n_lands);
        print_deck(&deck);

        assert_eq!(deck.total::<usize>(), 60);

        let now = Instant::now();
        let rv = sims(deck, &mut r, trials, turns);
        let elapsed = now.elapsed();
        let ms = (elapsed.as_micros() as f64) / 1000.;
        let ms_per_trial = ms / trials as f64;
        println!("{trials} simulations, elapsed {ms:.3} ms, {ms_per_trial:.3} ms/trial");

        for (key, counts) in rv.iter().sorted_by(|a, b| Ord::cmp(&a.0, &b.0)) {
            let mean = counts[counts.len() - 1].mean();
            let sem = counts[counts.len() - 1].ssem();
            println!("{key} {mean:.2} {sem:.2}")
        }
        println!("---\n");
        for i in 0..turns {
            n_lands_.push(n_lands);
            turn.push(i + 1); // making it 1-indexed for presentation
            lands.push(rv["lands"][i].mean());
            lands_sem.push(rv["lands"][i].ssem());
            power.push(rv["power"][i].mean());
            power_sem.push(rv["power"][i].ssem());
            hand.push(rv["hand"][i].mean());
            hand_sem.push(rv["hand"][i].ssem());
        }
        let last = turns - 1;
        for key in rv.keys() {
            fin_n_lands_.push(n_lands);
            fin_name.push(*key);
            fin_value.push(rv[key][last].mean());
            fin_value_sem.push(rv[key][last].ssem());
            fin_cumulative.push(key.starts_with("cumu_"));
        }
    }

    let data = object!{
        df: {
            n_lands: n_lands_,
            turn: turn,
            lands: lands,
            lands_sem: lands_sem,
            power: power,
            power_sem: power_sem,
            hand: hand,
            hand_sem: hand_sem,
        },
        final_df: {
            n_lands: fin_n_lands_,
            name: fin_name,
            value: fin_value,
            value_sem: fin_value_sem,
            cumulative: fin_cumulative,
        },
        plot: [
            ["lineplot", {hue: "n_lands", x: "turn", y: "lands", errbar: "lands_sem", data: "df"}],
            // ["lineplot", {hue: "n_lands", x: "turn", y: "cumu_lands", errbar: "cumu_lands_sem"}],
            ["lineplot", {hue: "n_lands", x: "turn", y: "power", errbar: "power_sem", data: "df"}],
            // ["lineplot", {hue: "n_lands", x: "turn", y: "cumu_power", errbar: "cumu_power_sem"}],
            ["catplot", {x: "n_lands", y: "value", hue: "name", data: "final_df", kind: "point", row: "cumulative", sharey: false, errbar: "value_sem"}],
            ["lineplot", {hue: "n_lands", x: "turn", y: "hand", data: "df"}],
        ],
    };
    fs::write("plot.json", data.dump()).unwrap();

    return ();
}

// ---
// optimization
// ---

// #[derive(PartialEq)]
// #[derive(Eq)]
// struct Entry<T: Eq> {
//     item: T,
//     priority: f64,
// }
// impl<T: Eq> PartialOrd for Entry<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(f64::total_cmp(&self.priority, &other.priority)) }
// }
// impl<T: Eq> Ord for Entry<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering { f64::total_cmp(&self.priority, &other.priority) }
// }


// pub struct SmallestK<T> {
//     size: usize,
//     max_heap: BinaryHeap<T>,
// }

// impl<T: Ord> SmallestK<T> {
//     fn new(size: usize) -> SmallestK<T> {
//         SmallestK {
//             size,
//             max_heap: BinaryHeap::with_capacity(size),
//         }
//     }

//     fn push(&mut self, item: &T) {
//         if self.max_heap.len() < self.size {
//             self.max_heap.push(*item);
//         } else {
//             let max = self.max_heap.peek().unwrap();
//             if item < max {
//                 self.max_heap.pop().unwrap();
//                 self.max_heap.push(*item);
//             }
//         }
//     }
// }

fn propose(r: &mut Rng, deck: &Counter<Card>) -> Counter<Card> {
    let mut new_deck = deck.clone();

    // Uniformly resample 1, 2, or 3 cards.
    let sample_count: usize = Categorical::new(&[0., 1., 1., 1.]).unwrap().sample(r);

    for _ in 0..sample_count {
        let old_card = sample_card(deck, r);

        let cards = deck.keys().cloned().sorted().collect::<Vec<Card>>();
        let dist = Categorical::new(&vec![1.; cards.len()]).unwrap();
        let idx: usize = dist.sample(r);
        let new_card = cards[idx];

        new_deck[&old_card] -= 1;
        new_deck[&new_card] += 1;
    }
    assert_eq!(deck.total::<usize>(), new_deck.total::<usize>());
    new_deck
}

fn acceptance_probability(old_energy: f64, new_energy: f64, temperature: f64) -> f64 {
    if new_energy < old_energy {
        1.
    } else {
        (-(new_energy - old_energy) / temperature).exp()
    }
}

fn accept(r: &mut Rng, old_energy: f64, new_energy: f64, temperature: f64) -> bool {
    let p = acceptance_probability(old_energy, new_energy, temperature);
    if p == 1. {
        true
    } else {
        let dist = Bernoulli::new(p).unwrap();
        dist.sample(r)
    }
}

fn energy(r: &mut Rng, deck: Counter<Card>, trials: usize, turns: usize) -> (f64, Counter<Card>) {
    let rv = sims(deck.clone(), r, trials, turns);
    let energy = -rv["cumu_power"].last().unwrap().mean();
    (energy, deck)
}

fn print_deck_stats(r: &mut Rng, deck: &Counter<Card>, turns: usize) {
    print_deck(deck);
    let rv = sims(deck.clone(), r, 10_000, turns);
    for (k, vec) in rv.iter().sorted_by(|a, b| Ord::cmp(&a.0, &b.0)) {
        let last = vec.last().unwrap();
        let mean = last.mean();
        let sem = last.ssem();
        println!("{k} {mean:.3} {sem:.3}");
    }
}

fn opti(deck: &str, chains: usize, log_every: usize, samples: usize, trials: usize, turns: usize) {
    // Keeping this immutable to reduce variance.
    let energy_rng = StdRng::seed_from_u64(9629878374);

    let mut r = StdRng::seed_from_u64(2347823);

    let start = energy(&mut energy_rng.clone(), make_deck(deck, 24), trials, turns);
    print_deck_stats(&mut energy_rng.clone(), &start.1, turns);
    let mut best = start.clone();
    // let mut s = SmallestK::new(3);

    for c in 0..chains {
        let mut x = start.clone();
        let mut accept_count = 0;
        println!("-- Chain {c} --");
        for iter in 0..samples {
            let deck = propose(&mut r, &x.1);
            let new_x = energy(&mut energy_rng.clone(), deck, trials, turns);
            let accept = accept(&mut r, x.0, new_x.0, 0.1);
            if accept {
                x = new_x;
                accept_count += 1;
            }
            if (iter + 1) % log_every == 0 {
                let energy = x.0;
                println!("{iter} accepted={accept} energy={energy}")
            }
            // s.push(&x);
            if x.0 < best.0 {
                best = x.clone();
            }
        }
        println!("---");
        let ratio = accept_count as f64 / samples as f64;
        println!("Acceptance ratio {ratio}");
    }
    println!("---");

    print_deck_stats(&mut energy_rng.clone(), &best.1, turns);
    // for best in s.max_heap.iter().sorted() {}
}

fn log_sim(deck: &str, turns: usize) {
    let mut r = Rng::seed_from_u64(38278347);
    sim(make_deck(deck, 24), &mut r, turns, true);
}

// ---
// CLI stuff
// ---

#[derive(clap::ValueEnum, Debug, Clone)]
enum Mode {
    Eval,
    Opti,
    LogSim,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_enum, default_value_t=Mode::Eval)]
    mode: Mode,

    #[arg(long, default_value="mono2")]
    deck: String,

    #[arg(long, default_value_t=3)]
    chains: usize,

    #[arg(long, default_value_t=50)]
    log_every: usize,

    #[arg(long, default_value_t=500)]
    samples: usize,

    #[arg(long, default_value_t=1_000)]
    trials: usize,

    #[arg(long, default_value_t=10)]
    turns: usize,
}

fn main() {
    let args = Args::parse();
    match args.mode {
        Mode::LogSim => log_sim(&args.deck, args.turns),
        Mode::Eval => eval(&args.deck, args.trials, args.turns),
        Mode::Opti => opti(&args.deck, args.chains, args.log_every, args.samples, args.trials, args.turns),
    }
}

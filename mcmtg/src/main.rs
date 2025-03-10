use std::ops::IndexMut;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, RangeInclusive};
use std::time::Instant;

use derivative::Derivative;
use json::object;
use meansd::MeanSD;
use rand::{Rng as Rng_, SeedableRng};
use statrs::distribution::{Bernoulli, Categorical};
use pathfinding::prelude::astar;
use ordered_float::OrderedFloat;
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use itertools::Itertools;
use clap::{Parser,command,arg};

type Rng = StdRng;

#[derive(Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
pub enum Card {
    Land,
    Other,
    Summon { cost: usize, power: usize, toughness: usize },
}

impl Card {
    pub fn cost(&self) -> Option<usize> {
        match self {
            Card::Summon { cost, power: _, toughness: _ } => Some(*cost),
            _ => None,
        }
    }

    pub fn power(&self) -> Option<usize> {
        match self {
            Card::Summon { cost: _, power, toughness: _ } => Some(*power),
            _ => None,
        }
    }
}

fn print_deck(deck: &Cards) {
    for (key, ct) in deck.iter().sorted() {
        if *ct == 0 {
            continue
        }
        println!("{ct} {key:?}");
    }
}

pub fn sample_categorical(r: &mut Rng, weights: &[usize]) -> usize {
    let total = weights.iter().sum();
    let mut sample_idx = r.gen_range(0..total);
    for (idx, w) in weights.iter().enumerate() {
        if sample_idx < *w {
            return idx
        }
        sample_idx -= *w;
    }
    unreachable!();
}

#[derive(Derivative)]
#[derivative(Hash)]
#[derive(Clone)]
pub struct FixedCounter<T> {
    items: Vec<T>,
    #[derivative(Hash="ignore")]
    index_map: HashMap<T, usize>,
    counter: Vec<usize>,
}

impl<T: Eq + Hash + Copy> FixedCounter<T> {
    fn new(items: Vec<T>) -> Self {
        let index_map = items.iter().enumerate().map(|(idx, &x)| (x, idx)).collect();
        let counter = (0..items.len()).map(|_x| 0).collect();
        Self {
            items,
            index_map,
            counter,
        }
    }

    fn from(cards: &[(T, usize)]) -> Self {
        let (items, counter): (Vec<T>, Vec<usize>) = cards.iter().cloned().unzip();
        let mut rv = Self::new(items);
        rv.counter = counter;
        rv
    }

    fn empty_clone(&self) -> Self {
        let mut clone = self.clone();
        clone.clear();
        clone
    }

    fn iter(&self) -> impl Iterator<Item=(&T, &usize)> {
        self.items.iter().zip(self.counter.iter())
    }

    fn clear(&mut self) {
        self.counter.fill(0);
    }

    fn total(&self) -> usize {
        self.counter.iter().sum()
    }

    fn get_entry(&self, item: &T) -> Option<FixedCounterEntry<T>> {
        if let Some(index) = self.index_map.get(item) {
            Some(FixedCounterEntry {
                index: *index,
                item: *item,
            })
        } else {
            None
        }
    }

    fn sample_item(&self, r: &mut Rng) -> FixedCounterEntry<T> {
        let index = sample_categorical(r, &self.counter);
        FixedCounterEntry { index, item: self.items[index] }
    }
}

pub struct FixedCounterEntry<T> {
    index: usize,
    item: T,
}

impl<T: Eq + Hash + Copy> Index<&T> for FixedCounter<T> {
    type Output = usize;
    fn index(&self, index: &T) -> &Self::Output {
        if let Some(idx) = self.index_map.get(index) {
            &self.counter[*idx]
        } else {
            &0
        }
    }
}
impl<T: Eq + Hash + Copy> IndexMut<&T> for FixedCounter<T> {
    fn index_mut(&mut self, index: &T) -> &mut Self::Output {
        self.counter.get_mut(self.index_map[index]).unwrap()
    }
}

impl<T: Eq + Hash + Copy> Index<&FixedCounterEntry<T>> for FixedCounter<T> {
    type Output = usize;
    fn index(&self, index: &FixedCounterEntry<T>) -> &Self::Output {
        &self.counter[index.index]
    }
}
impl<T: Eq + Hash + Copy> IndexMut<&FixedCounterEntry<T>> for FixedCounter<T> {
    fn index_mut(&mut self, index: &FixedCounterEntry<T>) -> &mut Self::Output {
        self.counter.get_mut(index.index).unwrap()
    }
}

impl<'a, T> AddAssign<&'a FixedCounter<T>> for FixedCounter<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        // TODO TODO HACK check if matched cards
        // TODO TODO HACK check if matched cards
        // TODO TODO HACK check if matched cards
        for (lhs, rhs) in self.counter.iter_mut().zip(rhs.counter.iter()) {
            *lhs += *rhs;
        }
    }
}

type Cards = FixedCounter<Card>;
type CardEntry = FixedCounterEntry<Card>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
// struct ManaSpend<'a> {
struct ManaSpend {
    // green: usize, // hack....
    // red: usize,
    // white: usize,
    // total: usize, // sum of above
    // color_specific_spend: ?,
    spend: usize,
    curr_idx: usize,
}

// impl<'a> ManaSpend<'_> {
//     fn new(cards: &'a [Card], counts: &'a [usize], total: usize) -> ManaSpend<'a> {
//         // HACK: assert same length
//         ManaSpend {
//             cards,
//             counts,
//             total,
//             spend: 0,
//             curr_idx: 0,
//         }
//     }

//     fn actions(&self) -> RangeInclusive<usize> {
//         if self.curr_idx == self.cards.len() || self.spend == self.total {
//             return 1..=0
//         }
//         let card = self.cards[self.curr_idx];
//         let cost = card.cost().unwrap_or(0);
//         if cost == 0 {
//             return 0..=0
//         }
//         // must be under total afterward
//         // and under # of cards beforehand
//         let left = self.total - self.spend;
//         let max_before_goal = left / cost;
//         let max_available = self.counts[self.curr_idx];
//         let limit = usize::min(max_available, max_before_goal);
//         0..=limit
//     }

//     fn next_state(&'a self, action: usize) -> ManaSpend<'a> {
//         let card = self.cards[self.curr_idx];
//         let cost = card.cost().unwrap_or(0);
//         ManaSpend {
//             cards: self.cards,
//             counts: self.counts,
//             total: self.total,
//             spend: self.spend + cost * action,
//             curr_idx: self.curr_idx + 1,
//         }
//     }

//     // fn next_states(&'a self) -> impl Iterator<Item = ManaSpend<'a>> {
//     // // fn next_state(&'a self, action: usize) -> ManaSpend<'a> {
//     // // the impl of `let rng` is definitely wrong. was trying to play with signatures here. but leaning against it b/c
//     // // it makes reconstructing path tricky
//     //     let card = self.cards[self.curr_idx];
//     //     let cost = card.cost().unwrap_or(0);
//     //     let rng = 0..=cost; // hack this is very wrong. just testing it out

//     //     rng.into_iter().map(move |action| {
//     //         ManaSpend {
//     //             cards: self.cards,
//     //             counts: self.counts,
//     //             total: self.total,
//     //             spend: self.spend + cost * action,
//     //             curr_idx: self.curr_idx + 1,
//     //         }
//     //     })
//     // }
// }

struct Problem<'a> {
    cards: &'a [Card],
    counts: &'a [usize],
    total: usize,
}

impl<'a> Problem<'_> {
    fn new(cards: &'a [Card], counts: &'a [usize], total: usize) -> Problem<'a> {
        assert_eq!(cards.len(), counts.len());
        Problem {
            cards,
            counts,
            total,
        }
    }

    fn initial_state(&self) -> ManaSpend {
        ManaSpend {
            spend: 0,
            curr_idx: 0,
        }
    }

    fn is_terminal(&self, state: &ManaSpend) -> bool {
        state.curr_idx == self.cards.len()
    }

    fn actions(&self, state: &ManaSpend) -> RangeInclusive<usize> {
        if self.is_terminal(state) {
            return 1..=0
        }
        if state.spend == self.total {
            return 0..=0
        }
        let card = self.cards[state.curr_idx];
        let cost = card.cost().unwrap_or(0);
        if cost == 0 {
            return 0..=0
        }
        // must be under total afterward
        // and under # of cards beforehand
        let left = self.total - state.spend;
        let max_before_goal = left / cost;
        let max_available = self.counts[state.curr_idx];
        let limit = usize::min(max_available, max_before_goal);
        0..=limit
    }

    fn next_state(&self, state: &ManaSpend, action: usize) -> (ManaSpend, usize) {
        let card = self.cards[state.curr_idx];
        let cost = card.cost().unwrap_or(0);
        (
            ManaSpend {
                spend: state.spend + cost * action,
                curr_idx: state.curr_idx + 1,
            },
            card.power().unwrap_or(0) * action,
        )
    }

    // fn next_states(&'a self) -> impl Iterator<Item = ManaSpend<'a>> {
    // // fn next_state(&'a self, action: usize) -> ManaSpend<'a> {
    // // the impl of `let rng` is definitely wrong. was trying to play with signatures here. but leaning against it b/c
    // // it makes reconstructing path tricky
    //     let card = self.cards[self.curr_idx];
    //     let cost = card.cost().unwrap_or(0);
    //     let rng = 0..=cost; // hack this is very wrong. just testing it out

    //     rng.into_iter().map(move |action| {
    //         ManaSpend {
    //             cards: self.cards,
    //             counts: self.counts,
    //             total: self.total,
    //             spend: self.spend + cost * action,
    //             curr_idx: self.curr_idx + 1,
    //         }
    //     })
    // }
}

fn opti_mana_spend(p: Problem<'_>) -> Vec<usize> {
    let best_power_per_cost = p.cards.iter().filter_map(|c| {
        if let (Some(p), Some(cost)) = (c.power(), c.cost()) {
            Some(OrderedFloat(p as f64 / cost as f64))
        } else {
            None
        }
    }).max().unwrap();

    let result = astar(
        &p.initial_state(),
        |s| {
            let s = s.clone();
            let p_ref = &p;
            p.actions(&s).map(move |a| {
                let (ns, r) = p_ref.next_state(&s, a);
                (ns, -OrderedFloat(r as f64))
            })
        },
        |s| {
            if p.is_terminal(s) {
                OrderedFloat(0.)
            } else {
                let left = p.total - s.spend;
                -OrderedFloat(left as f64) * best_power_per_cost
            }
        },
        |s| p.is_terminal(s),
    );
    let (plan, _cost) = result.unwrap();
    plan.iter().tuple_windows().map(|(curr, next)| {
        let change = next.spend - curr.spend;
        if change == 0 {
            return 0
        }
        let card = p.cards[curr.curr_idx];
        let cost = card.cost().unwrap();
        assert_eq!(change % cost, 0);
        change / cost
    }).collect()
}

pub struct FieldStats {
    lands: usize,
    power: usize,
    toughness: usize,
    summons: usize,
    hand: usize,
}

pub struct Field {
    pub deck: Cards,
    pub hand: Cards,
    pub tapped: Cards,
    pub untapped: Cards,
}

impl Field {
    pub fn new(deck: Cards) -> Field {
        Field {
            hand: deck.empty_clone(),
            tapped: deck.empty_clone(),
            untapped: deck.empty_clone(),
            deck,
        }
    }

    pub fn draw_card(&mut self, r: &mut Rng) -> CardEntry {
        let c = self.deck.sample_item(r);
        self.deck[&c] -= 1;
        self.hand[&c] += 1;
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
        self.untapped += &self.tapped;
        self.tapped.clear();
    }

    pub fn played_stats(&self) -> FieldStats {
        let mut lands = 0;
        let mut power = 0;
        let mut toughness = 0;
        let mut summons = 0;
        for played in [&self.untapped, &self.tapped] {
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
        }
        let hand = self.hand.total();
        FieldStats { lands, power, toughness, summons, hand }
    }

    pub fn end(&mut self) {}

    pub fn play_card(&mut self, card: Card) -> Option<()> {
        let land = self.deck.get_entry(&Card::Land).unwrap();
        if let Some(card) = self.deck.get_entry(&card) {
            self.play(&card, &land)
        } else {
            None
        }
    }

    pub fn play(&mut self, card: &CardEntry, mana_source: &CardEntry) -> Option<()> {
        if self.hand[card] == 0 {
            return None
        }
        match card.item {
            Card::Land => {
                // Move card
                self.hand[card] -= 1;
                self.untapped[card] += 1;
            },
            Card::Summon { cost, power: _, toughness: _ } => {
                if self.untapped[mana_source] < cost {
                    return None
                }
                // Move card
                self.hand[card] -= 1;
                self.tapped[card] += 1;
                // Pay cost
                self.untapped[mana_source] -= cost;
                self.tapped[mana_source] += cost;
            }
            Card::Other => unreachable!("should not play Card::Other"),
        }
        Some(())
    }

    pub fn play_card_while_possible(&mut self, card: Card) {
        let land = self.deck.get_entry(&Card::Land).unwrap();
        if let Some(card) = self.deck.get_entry(&card) {
            self.play_while_possible(&card, &land)
        }
    }
    pub fn play_while_possible(&mut self, card: &CardEntry, mana_source: &CardEntry) {
        let mut res = self.play(card, mana_source);
        while res.is_some() {
            res = self.play(card, mana_source);
        }
    }

    pub fn play_opti(&mut self) {
        let cards = self.hand.items.clone();
        let counts = self.hand.counter.clone();
        let total = self.untapped[&Card::Land];
        let p = Problem::new(&cards, &counts, total);
        let actions = opti_mana_spend(p);
        for (card, ct) in cards.iter().zip(actions) {
            for _ in 0..ct {
                self.play_card(*card).unwrap();
            }
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

    fn assert_eq_ct(a: &Cards, b: &Cards) {
        let mut keys: HashSet<Card> = a.items.iter().cloned().collect();
        keys.extend(b.items.iter());
        for key in keys {
            assert_eq!(a[&key], b[&key], "Testing for {key:?}");
        }
    }

    #[test]
    fn test_field() {
        let c1 = Card::Summon { cost: 1, power: 1, toughness: 1 };
        let c2 = Card::Summon { cost: 2, power: 2, toughness: 2 };
        let l = Card::Land;
        let deck: Cards = Cards::from(&[
            (l, 40),
            (c1, 20),
            (c2, 0),
        ]);
        assert_eq!(deck.total(), 60);
        let mut field = Field::new(deck);
        let mut r = StdRng::seed_from_u64(3667);

        field.hand[&l] += 3;
        field.hand[&c1] += 2;
        field.hand[&c2] += 2;

        assert_eq!(field.hand.total(), 7);
        field.begin(&mut r);
        assert_eq!(field.tapped.total(), 0);
        assert_eq!(field.untapped.total(), 0);
        field.play_card(l);
        assert_eq!(field.tapped.total(), 0);
        assert_eq_ct(&field.untapped, &Cards::from(&[(l, 1)]));
        field.play_card_while_possible(c1);
        assert_eq_ct(&field.tapped, &Cards::from(&[(l, 1), (c1, 1)]));
        assert_eq!(field.untapped.total(), 0);

        assert_eq!(field.hand.total(), 6);
        field.begin(&mut r);
        assert_eq!(field.tapped.total(), 0);
        assert_eq_ct(&field.untapped, &Cards::from(&[(l, 1), (c1, 1)]));
        field.play_card(l);
        assert_eq!(field.tapped.total(), 0);
        assert_eq_ct(&field.untapped, &Cards::from(&[(l, 2), (c1, 1)]));
        field.play_card_while_possible(c2);
        field.play_card_while_possible(c1);
        assert_eq_ct(&field.tapped, &Cards::from(&[(l, 2), (c2, 1)]));
        assert_eq_ct(&field.untapped, &Cards::from(&[(c1, 1), (l, 0)]));

        assert_eq!(field.played_stats().power, 3);

        assert_eq!(field.hand.total(), 5);
        field.begin(&mut r);
        field.play_card(l);
        field.play_card_while_possible(c2);
        field.play_card_while_possible(c1);
        assert_eq_ct(&field.tapped, &Cards::from(&[(l, 3), (c1, 1), (c2, 1)]));
        assert_eq_ct(&field.untapped, &Cards::from(&[(l, 0), (c1, 1), (c2, 1)]));

        assert_eq!(field.played_stats().power, 6);

        assert_eq!(field.hand.total(), 3);
    }

    #[test]
    fn test_cumulative_sum() {
        assert_eq!(cumulative_sum(&vec![1, 4, 2, 5]), vec![1, 5, 7, 12])
    }

    #[test]
    fn test_mana_spend() {
        let cards = [
            Card::Summon { cost: 1, power: 1, toughness: 1 },
            Card::Summon { cost: 1, power: 1, toughness: 1 },
            Card::Land,
            Card::Summon { cost: 2, power: 2, toughness: 2 },
            Card::Summon { cost: 3, power: 3, toughness: 3 },
        ];
        let counts = [2, 6, 1, 2, 2];
        let p = Problem::new(&cards, &counts, 4);

        // Listens to counts
        let s0: ManaSpend = p.initial_state();
        assert_eq!(p.actions(&s0), 0..=2);
        // assert_eq!(&s0.next_states().map(|x| x.spend).collect::<Vec<usize>>(), vec![0, 1, 2]);

        // Listens to total
        let (s1, _) = p.next_state(&s0, 0);
        assert_eq!(p.actions(&s1), 0..=4);

        // Ignore lands
        let (s2, _) = p.next_state(&s1, 0);
        assert_eq!(p.actions(&s2), 0..=0);

        let (s3, _) = p.next_state(&s2, 0);
        assert_eq!(p.actions(&s3), 0..=2);

        // not evenly divisible
        let (s4, _) = p.next_state(&s3, 0);
        assert_eq!(p.actions(&s4), 0..=1);

        // end
        let (s5, _) = p.next_state(&s4, 0);
        assert_eq!(p.actions(&s5), 1..=0);
        assert!(p.actions(&s5).is_empty());

        // rewinding. actions should depend on what's left to fill
        let (s1, _) = p.next_state(&s0, 2);
        assert_eq!(s1.spend, 2);
        assert_eq!(p.actions(&s1), 0..=2);

        // If all spent, automatic 0
        let (s2, _) = p.next_state(&s1, 2);
        assert_eq!(s2.spend, 4);
        assert_eq!(p.actions(&s2), 0..=0);
    }

    #[test]
    fn test_opti_mana_spend() {
        // Making sure we get 2x of a 2 instead of 1x of a 3
        let cards = [
            Card::Summon { cost: 5, power: 5, toughness: 5 },
            Card::Summon { cost: 3, power: 3, toughness: 3 },
            Card::Summon { cost: 2, power: 2, toughness: 2 },
            Card::Land,
        ];
        assert_eq!(opti_mana_spend(Problem::new(&cards, &[1, 2, 2, 0], 4)), vec![0, 0, 2, 0]);

        let cards = [
            // Including this b/c it throws off the power/cost ratio.
            Card::Summon { cost: 5, power: 10, toughness: 10 },
            Card::Summon { cost: 3, power: 3, toughness: 3 },
            Card::Summon { cost: 2, power: 2, toughness: 2 },
            Card::Land,
        ];
        assert_eq!(opti_mana_spend(Problem::new(&cards, &[1, 2, 2, 0], 4)), vec![0, 0, 2, 0]);

        // Making sure we keep considering better options
        let cards = [
            Card::Summon { cost: 2, power: 2, toughness: 2 },
            Card::Summon { cost: 3, power: 4, toughness: 4 },
            Card::Summon { cost: 1, power: 1, toughness: 1 },
        ];
        assert_eq!(opti_mana_spend(Problem::new(&cards, &[2, 1, 1], 4)), vec![0, 1, 1]);

        // Finding examples that don't fully spend but are better
        let cards = [
            Card::Summon { cost: 5, power: 7, toughness: 7 },
            Card::Summon { cost: 2, power: 2, toughness: 2 },
        ];
        assert_eq!(opti_mana_spend(Problem::new(&cards, &[1, 3], 6)), vec![1, 0]);
    }

    #[test]
    fn test_sample_categorical() {
        let mut r = StdRng::seed_from_u64(3663);
        let mut cts = [0; 4];
        for _ in 0..10000 {
            cts[sample_categorical(&mut r, &[5, 2, 1, 2])] += 1;
        }
        assert_eq!(cts, [5009, 1987, 991, 2013]);
    }

    #[test]
    fn test_smallest_k() {
        let mut s = SmallestK::new(3);
        s.push(3, 3);
        s.push(-1, -1);
        assert_eq!(s.items().sorted().collect::<Vec<i32>>(), vec![-1, 3]);

        s.push(2, 2);
        assert_eq!(s.items().sorted().collect::<Vec<i32>>(), vec![-1, 2, 3]);

        // ejects non-smallest, maintains size
        s.push(1, 1);
        assert_eq!(s.items().sorted().collect::<Vec<i32>>(), vec![-1, 1, 2]);

        // ignores if not smaller, maintains size
        s.push(10, 10);
        assert_eq!(s.items().sorted().collect::<Vec<i32>>(), vec![-1, 1, 2]);
    }
}


fn sim(deck: Cards, r: &mut Rng, turns: usize, log: bool) -> (Field, Vec<FieldStats>) {
    let mana_source = deck.get_entry(&Card::Land).unwrap();
    let summons: Vec<CardEntry> = (0..10).rev().filter_map(|n|
        deck.get_entry(&Card::Summon { cost: n, power: n, toughness: n })
    ).collect();
    let mut field = Field::new(deck);
    let mut stats = Vec::with_capacity(turns);
    field.init(r);
    for t in 0..turns {
        field.begin(r);
        field.play(&mana_source, &mana_source);

        for c in &summons {
            field.play_while_possible(c, &mana_source);
        }
        // field.play_opti();

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

fn sims(deck: Cards, r: &mut Rng, trials: usize, turns: usize) -> HashMap<&'static str, Vec<MeanSD>> {
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

pub fn make_deck(name: &str, n_lands: usize) -> Cards {
    let c1 = Card::Summon { cost: 1, power: 1, toughness: 1 };
    let c2 = Card::Summon { cost: 2, power: 2, toughness: 2 };
    let c3 = Card::Summon { cost: 3, power: 3, toughness: 3 };
    let c4 = Card::Summon { cost: 4, power: 4, toughness: 4 };
    let c5 = Card::Summon { cost: 5, power: 5, toughness: 5 };
    let c6 = Card::Summon { cost: 6, power: 6, toughness: 6 };

    let deck = match name {
        "mono" => {
            // sort of based on a mono-red deck
            // originally, it's 24 lands, 24 creatures (12x1, 8x2, 4x3), rest spells
            // https://mtga.untapped.gg/meta/decks/510/mono-red-aggro/AAQAAQABiKIyAaXULArswgHGkAHbEt7tKbTvBBgIELUDjwcBD-IJAA?tab=overview
            let n_creatures = 60 - n_lands;
            let mut deck = Cards::from(&[
                (Card::Land, n_lands),
                (c1, n_creatures / 2),
                (c2, n_creatures / 3),
                (c3, n_creatures / 6),
                (Card::Other, 0),
            ]);
            assert!(deck.total() <= 60);
            deck[&Card::Other] += 60 - deck.total();
            deck
        },
        "mono2" => {
            // sort of based on this, took middle of their ranges
            // https://magic.wizards.com/en/news/feature/how-build-mana-curve-2017-05-18
            // 1 + 5 + 4 + 3 + 2 + 1 = 16
            let n_creatures = 60 - n_lands;
            let mut deck = Cards::from(&[
                (Card::Land, n_lands),
                (c1, (n_creatures as f64 * 1./16.).floor() as usize),
                (c2, (n_creatures as f64 * 5./16.).floor() as usize),
                (c3, (n_creatures as f64 * 4./16.).floor() as usize),
                (c4, (n_creatures as f64 * 3./16.).floor() as usize),
                (c5, (n_creatures as f64 * 2./16.).floor() as usize),
                (c6, (n_creatures as f64 * 1./16.).floor() as usize),
                (Card::Other, 0),
            ]);
            assert!(deck.total() <= 60);
            deck[&Card::Other] += 60 - deck.total();
            deck
        },
        "pow1" => {
            Cards::from(&[
                (Card::Land, n_lands),
                (c1, 60 - n_lands),
            ])
        },
        // "fk_aggro" => {
        //     // Would be nice to replicate this: https://www.peasant-magic.com/articles/magic-deckbuilding/finding-the-optimal-aggro-deck-via-computer
        //     let c1 = Card::Summon { cost: 1, power: 2, toughness: 2 };
        //     deck[&c1] += 10;
        //     let c2 = Card::Summon { cost: 2, power: 4, toughness: 4 };
        //     deck[&c2] += 10;
        //     let c3 = Card::Summon { cost: 3, power: 6, toughness: 6 };
        //     deck[&c3] += 10;
        //     assert!(deck.total() <= 60);
        // },
        _ => unreachable!(),
    };
    assert_eq!(deck.total(), 60);
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

        assert_eq!(deck.total(), 60);

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


pub struct SmallestK<T, O> {
    size: usize,
    max_heap: BinaryHeap<(O, usize)>,
    items: HashMap<usize, T>,
    index: usize,
}

impl<T: Clone, O: Clone + Ord> SmallestK<T, O> {
    fn new(size: usize) -> SmallestK<T, O> {
        SmallestK {
            size,
            max_heap: BinaryHeap::with_capacity(size),
            items: HashMap::with_capacity(size),
            index: 0,
        }
    }

    fn push(&mut self, item: T, score: O) {
        if self.max_heap.len() < self.size {
            self.max_heap.push((score, self.index));
            self.items.insert(self.index, item);
            self.index += 1
        } else {
            let max = self.max_heap.peek().unwrap();
            if score < max.0 {
                // Remove max entry
                let entry = self.max_heap.pop().unwrap();
                self.items.remove(&entry.1).unwrap();
                // Add new entry
                self.max_heap.push((score, self.index));
                self.items.insert(self.index, item);
                self.index += 1
            }
        }
        assert!(self.max_heap.len() == self.items.len());
    }

    fn items(&self) -> impl Iterator<Item=T> {
        self.max_heap.iter().map(|(_score, index)| self.items[index].clone())
    }
}

fn propose(r: &mut Rng, deck: &Cards) -> Cards {
    let mut new_deck = deck.clone();

    // Uniformly resample 1, 2, or 3 cards.
    let sample_count: usize = Categorical::new(&[0., 1., 1., 1.]).unwrap().sample(r);

    for _ in 0..sample_count {
        let old_card = deck.sample_item(r);

        let cards = &deck.items;
        let dist = Categorical::new(&vec![1.; cards.len()]).unwrap();
        let idx: usize = dist.sample(r);
        let new_card = cards[idx];

        new_deck[&old_card] -= 1;
        new_deck[&new_card] += 1;
    }
    assert_eq!(deck.total(), new_deck.total());
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

fn energy(r: &mut Rng, deck: Cards, trials: usize, turns: usize) -> (f64, Cards) {
    let rv = sims(deck.clone(), r, trials, turns);
    let energy = -rv["cumu_power"].last().unwrap().mean();
    (energy, deck)
}

fn print_deck_stats(r: &mut Rng, deck: &Cards, turns: usize) {
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
    let mut s = SmallestK::new(3);
    let now = Instant::now();

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
            s.push(x.1.clone(), OrderedFloat(x.0));
            if x.0 < best.0 {
                best = x.clone();
            }
        }
        println!("---");
        let ratio = accept_count as f64 / samples as f64;
        println!("Acceptance ratio {ratio}");
    }
    println!("---");

    let ms = (now.elapsed().as_micros() as f64) / 1000.;
    let total_samples = chains * samples;
    let ms_per_trial = ms / total_samples as f64;
    println!("{total_samples} samples, elapsed {ms:.3} ms, {ms_per_trial:.3} ms/sample");

    print_deck_stats(&mut energy_rng.clone(), &best.1, turns);
    for best in s.items() {
        print_deck_stats(&mut energy_rng.clone(), &best, turns);
    }
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

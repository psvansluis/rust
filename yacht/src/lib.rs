use itertools::Itertools;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => sum_of_side(1, dice),
        Category::Twos => sum_of_side(2, dice),
        Category::Threes => sum_of_side(3, dice),
        Category::Fours => sum_of_side(4, dice),
        Category::Fives => sum_of_side(5, dice),
        Category::Sixes => sum_of_side(6, dice),
        Category::FullHouse => full_house(dice),
        Category::FourOfAKind => four_of_a_kind(dice),
        Category::LittleStraight => little_straight(dice),
        Category::BigStraight => big_straight(dice),
        Category::Choice => choice(dice),
        Category::Yacht => yacht(dice),
    }
}

fn unique_sides(dice: Dice) -> Vec<u8> {
    dice.into_iter().unique().collect::<Vec<u8>>()
}

fn count_unique(dice: Dice) -> u8 {
    unique_sides(dice).len() as u8
}

fn count_side(target_side: u8, dice: Dice) -> u8 {
    dice.into_iter().filter(|&side| side == target_side).count() as u8
}

fn four_of_a_kind(dice: Dice) -> u8 {
    match unique_sides(dice)
        .into_iter()
        .find(|&side| count_side(side, dice) >= 4)
    {
        Some(side) => side * 4,
        None => 0,
    }
}

fn full_house(dice: Dice) -> u8 {
    match unique_sides(dice)
        .into_iter()
        .map(|side| count_side(side, dice))
        .collect::<Vec<u8>>()[..]
    {
        [2, 3] | [3, 2] => choice(dice),
        _ => 0,
    }
}

fn little_straight(dice: Dice) -> u8 {
    match (count_unique(dice), count_side(6, dice)) {
        (5, 0) => 30,
        _ => 0,
    }
}

fn big_straight(dice: Dice) -> u8 {
    match (count_unique(dice), count_side(1, dice)) {
        (5, 0) => 30,
        _ => 0,
    }
}

fn yacht(dice: Dice) -> u8 {
    match count_unique(dice) {
        1 => 50,
        _ => 0,
    }
}

fn sum_of_side(target_side: u8, dice: Dice) -> u8 {
    target_side * count_side(target_side, dice)
}

fn choice(dice: Dice) -> u8 {
    dice.iter().sum()
}

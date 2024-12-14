use std::ops::MulAssign;

struct Game {
    button_a: Coords,
    button_b: Coords,
    prize: Coords,
}

struct Coords {
    x: i64,
    y: i64,
}

#[inline]
fn parse_input(input: &str) -> impl Iterator<Item = Game> + use<'_> {
    input.split("\n\n").map(|game| {
        let mut coord_list = game.split("\n");
        let button_a = parse_button_line(coord_list.next().unwrap());
        let button_b = parse_button_line(coord_list.next().unwrap());
        let prize = parse_prize_line(coord_list.next().unwrap());
        Game {
            button_a,
            button_b,
            prize,
        }
    })
}

#[inline]
fn parse_prize_line(line: &str) -> Coords {
    let comma_index = line.bytes().position(|b| b == b',').unwrap();
    let x = line[9..comma_index].parse().unwrap();
    let y = line[comma_index + 4..].parse().unwrap();
    Coords { x, y }
}

#[inline]
fn parse_button_line(line: &str) -> Coords {
    let bytes = line.as_bytes();
    let x = parse_2_dig_int(bytes[12], bytes[13]);
    let y = parse_2_dig_int(bytes[18], bytes[19]);
    Coords { x, y }
}

#[inline]
fn parse_2_dig_int(one: u8, two: u8) -> i64 {
    (one - b'0') as i64 * 10 + (two - b'0') as i64
}

struct Equation {
    b: i64,
    prize: i64,
}

impl Equation {
    #[inline]
    fn new(b: i64, prize: i64) -> Self {
        Self { b, prize }
    }
}

impl MulAssign<i64> for Equation {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        self.b *= rhs;
        self.prize *= rhs;
    }
}

#[inline]
fn solve(games: impl Iterator<Item = Game>, limit100: bool) -> i64 {
    games
        .map(|g| {
            let mut x_eq = Equation::new(g.button_b.x, g.prize.x);
            let mut y_eq = Equation::new(g.button_b.y, g.prize.y);
            x_eq *= g.button_a.y;
            y_eq *= g.button_a.x;

            let b_rem = x_eq.b - y_eq.b;
            let prize_rem = x_eq.prize - y_eq.prize;
            let b_res = prize_rem / b_rem;
            if b_res <= 0 || prize_rem % b_rem != 0 || (limit100 && b_res > 100) {
                return 0;
            }

            let a_res = (g.prize.x - g.button_b.x * b_res) / g.button_a.x;
            if a_res <= 0
                || (g.prize.x - g.button_b.x * b_res) % g.button_a.x != 0
                || (limit100 && b_res > 100)
            {
                return 0;
            }
            a_res * 3 + b_res
        })
        .sum()
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let games = parse_input(input);
    solve(games, true)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let games = parse_input(input).map(|mut g| {
        g.prize.x += 10000000000000;
        g.prize.y += 10000000000000;
        g
    });
    solve(games, false)
}

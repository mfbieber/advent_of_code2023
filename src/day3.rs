use std::path::PathBuf;
use crate::read_lines;
pub struct Number {
    number: i32,
    x: i32,
    y: i32
}

struct Symbol {
    symbol: String,
    x: i32,
    y: i32
}

impl Number {
    fn length(&self) -> i32 {
        return self.number.to_string().len() as i32
    }

    fn is_symbol_adjacent(&self, symbol: &Symbol) -> bool {
        if symbol.y == self.y {
            if let Some(value) = self.is_number_next_to_symbol(&symbol) {
                return value;
            }
            return false;
        } else if symbol.y + 1  == self.y || symbol.y == self.y + 1 {
            if let Some(value) = self.is_number_next_to_symbol(&symbol) {
                return value;
            }
        }
        return false;
    }

    fn is_number_next_to_symbol(&self, symbol: &Symbol) -> Option<bool> {
        if symbol.x == self.x && symbol.y == self.y {
            return Some(false);
        }
        let mut i: i32 = 0;
        while i <= self.length() + 1 {
            if symbol.x + 1 == self.x + i {
                return Some(true);
            }
            i += 1;
        }
        return Some(false);
    }
}

fn day3_part1(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    // File input.txt must exist in the current path
    parse_lines_to_numbers_and_symbols(path, &mut symbols, &mut numbers);
    numbers.iter().for_each(|number| {
        for symbol in &symbols {
            if number.is_symbol_adjacent(symbol) {
                sum = sum + number.number;
                break;
            }
        }
    });
    println!("{}", sum);
    return sum;
}

fn parse_lines_to_numbers_and_symbols(path: &PathBuf, symbols: &mut Vec<Symbol>, mut numbers: &mut Vec<Number>) {
    if let Ok(lines) = read_lines(path) {
        let mut y: i32 = 0;
        for line in lines {
            let chars: Vec<&str> = line.as_ref().unwrap().split("").collect();
            let mut x: i32 = 0;
            let mut num_vec: Vec<i32> = vec![];
            const RADIX: i32 = 10;
            for char in chars {
                if char.eq(".") {
                    if num_vec.len() > 0 {
                        push_number(&mut numbers, &mut y, &mut x, &mut num_vec);
                        num_vec = vec![];
                    }
                    x += 1;
                    continue
                }
                let num = char.parse();
                if num.is_ok() {
                    num_vec.push(num.unwrap());
                    x += 1;
                    continue
                }
                if !char.eq("") {
                    let symbol: Symbol = Symbol {
                        symbol: char.to_string(),
                        x: x - 1,
                        y,
                    };
                    symbols.push(symbol);
                }
                if num_vec.len() > 0 {
                    push_number(&mut numbers, &mut y, &mut x, &mut num_vec);
                    num_vec = vec![];
                }
                x += 1;
            }
            if num_vec.len() > 0 {
                push_number(&mut numbers, &mut y, &mut x, &mut num_vec);
                num_vec = vec![];
            }
            y += 1;
        }
    }
}

fn push_number(numbers: &mut Vec<Number>, y: &mut i32, x: &mut i32, num_vec: &mut Vec<i32>) {
    let mut num: i32 = 0;
    let base: i32 = 10;
    let mut i: usize = num_vec.len();
    while i > 0 {
        num = num + num_vec.get(i - 1).unwrap() * base.pow((num_vec.len() - i) as u32);
        i -= 1;
    }
    let number: Number = Number {
        number: num,
        x : *x - num_vec.len() as i32 - 1,
        y: *y,
    };
    numbers.push(number);
}

fn day3_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    // File input.txt must exist in the current path
    parse_lines_to_numbers_and_symbols(path, &mut symbols, &mut numbers);
    let gears: Vec<Symbol> = symbols.into_iter().filter(|symbol| {
        symbol.symbol.eq("*")
    }).collect::<Vec<Symbol>>();

    gears.iter().for_each(|gear| {
        let mut gears: i32 = 0;
        let mut gear_ratio: i32 = 0;
        for number in &numbers {
            if number.is_symbol_adjacent(gear) {
                if gear_ratio == 0 {
                    gear_ratio = number.number;
                } else {
                    gear_ratio = gear_ratio * number.number;
                }
                gears += 1;
            }
            if gears == 2 {
                sum = sum + gear_ratio;
                gear_ratio = 0;
                gears = 0;
                break;
            }
        }
    });
    println!("{}", sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day3::{day3_part1, day3_part2, Number, Symbol};

    #[test]
    fn test_is_length_correct() {
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 0,
        };
        assert_eq!(number.length(), 3);
    }

    #[test]
    fn test_symbol_is_not_adjacent_left_larger_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 4,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 2,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_is_symbol_adjacent_left_larger_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 3,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 2,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_symbol_is_not_adjacent_left_smaller_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 2,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_is_symbol_adjacent_left_smaller_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 1,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_is_symbol_adjacent_left_same_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 0,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_is_symbol_not_adjacent_left_same_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 2,
            y: 0,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_is_symbol_adjacent_right_same_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 4,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 0,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_is_symbol_adjacent_bottom() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 2,
            y: 2,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 1,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_is_symbol_adjacent_top() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 2,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 1,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), true);
    }

    #[test]
    fn test_is_symbol_not_adjacent_right_same_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 5,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 1,
            y: 0,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_is_symbol_adjacent_same_x() {
        let symbol: Symbol = Symbol {
            symbol: String::from("#"),
            x: 6,
            y: 3,
        } ;
        let number: Number = Number {
            number: 633,
            x: 6,
            y: 2,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_is_symbol_adjacent_same_x_and_y() {
        let symbol: Symbol = Symbol {
            symbol: String::from("+"),
            x: 0,
            y: 0,
        } ;
        let number: Number = Number {
            number: 123,
            x: 0,
            y: 0,
        };
        assert_eq!(number.is_symbol_adjacent(&symbol), false);
    }

    #[test]
    fn test_part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day3/test/input.txt");
        assert_eq!(day3_part1(&d), 4361);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day3/input.txt");
        assert_eq!(day3_part1(&d), 543867);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day3/test/input.txt");
        assert_eq!(day3_part2(&d), 467835);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day3/input.txt");
        assert_eq!(day3_part2(&d), 79613331);
    }

}
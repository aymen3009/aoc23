use std::{convert::TryInto, fs::read_to_string};

struct NumberInfo {
    line: i32,
    col_start: i32,
    col_end: i32,
    number: String,
}
struct SymbolInfo {
    line: i32,
    col: i32,
    symbol: String,
}

fn main() {
    let filename = "input";
    let mut number_info: Vec<NumberInfo> = Vec::new();
    let mut symbol_info: Vec<SymbolInfo> = Vec::new();
    for (line, line_txt) in read_to_string(filename).unwrap().lines().enumerate() {
        let mut number = "".to_string();
        line_txt.chars().enumerate().for_each(|(col, c)| {
            if c.is_digit(10) {
                number.push(c);
            } else {
                if number != "" {
                    number_info.push(NumberInfo {
                        line: line.try_into().unwrap(),
                        col_start: (col - number.len()).try_into().unwrap(),
                        col_end: (col - 1).try_into().unwrap(),
                        number: number.clone(),
                    });
                    number = "".to_string();
                }
                if c != '.' {
                    symbol_info.push(SymbolInfo {
                        line: line.try_into().unwrap(),
                        col: (col).try_into().unwrap(),
                        symbol: c.to_string(),
                    });
                }
            }
        });
        if number != "" {
            number_info.push(NumberInfo {
                line: line.try_into().unwrap(),
                col_start: (line_txt.len() - number.len()).try_into().unwrap(),
                col_end: (line_txt.len() - 1).try_into().unwrap(),
                number: number.clone(),
            });
        }
    }
    let part_one_res = part_one(&number_info, &symbol_info);
    let part_two_res = part_two(&number_info, &symbol_info);
    println!("part one:{}", part_one_res);
    println!("part two:{}", part_two_res);
}

fn part_one(number_info: &Vec<NumberInfo>, symbol_info: &Vec<SymbolInfo>) -> i32 {
    let mut total = 0;
    for num in number_info {
        for sym in symbol_info {
            if sym.line <= num.line + 1 && sym.line >= num.line - 1 {
                if sym.col <= num.col_end + 1 && sym.col >= num.col_start - 1 {
                    total = total + num.number.parse::<i32>().unwrap();
                    break;
                }
            }
        }
    }
    total
}

fn part_two(number_info: &Vec<NumberInfo>, symbol_info: &Vec<SymbolInfo>) -> i32 {
    let mut total = 0;
    for sym in symbol_info {
        if sym.symbol != "*" {
            continue;
        }
        let mut ad_sym: Vec<i32> = Vec::new();
        for num in number_info {
            if sym.line <= num.line + 1 && sym.line >= num.line - 1 {
                if sym.col <= num.col_end + 1 && sym.col >= num.col_start - 1 {
                    ad_sym.push(num.number.parse::<i32>().unwrap());
                }
            }
        }
        if ad_sym.len() == 2 {
            total = total + (ad_sym[0] * ad_sym[1]);
        }
    }
    total
}

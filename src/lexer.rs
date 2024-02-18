#[derive(Debug)]
pub enum Token {
    Increment(u8),
    Decrement(u8),
    Forwards(u32),
    Backwards(u32),
    Print,
    LoopStart(usize),
    LoopEnd(usize),
}

pub fn parse(input: &str) -> Vec<Token> {
    let code = remove_comments(input);
    let mut tokens: Vec<Token> = Vec::new();
    let mut loop_end_positions: Vec<usize> = Vec::new();
    for (pos, c) in code.chars().enumerate() {
        match c {
            '>' => {
                if let Some(token) = tokens.last_mut() {
                    if let Token::Forwards(amt) = token {
                        *amt += 1;
                        continue;
                    }
                }
                tokens.push(Token::Forwards(1))
            }
            '<' => {
                if let Some(token) = tokens.last_mut() {
                    if let Token::Backwards(amt) = token {
                        *amt += 1;
                        continue;
                    }
                }
                tokens.push(Token::Backwards(1))
            }
            '+' => {
                if let Some(token) = tokens.last_mut() {
                    if let Token::Increment(amt) = token {
                        *amt += 1;
                        continue;
                    }
                }
                tokens.push(Token::Increment(1));
            }
            '-' => {
                if let Some(token) = tokens.last_mut() {
                    if let Token::Decrement(amt) = token {
                        *amt += 1;
                        continue;
                    }
                }
                tokens.push(Token::Decrement(1));
            }
            '.' => tokens.push(Token::Print),
            ',' => panic!("read not implemented"),
            '[' => {
                tokens.push(Token::LoopStart(loop_end_positions.len()));
                loop_end_positions.push(find_matching_close(input, pos));
            }
            ']' => {
                tokens.push(Token::LoopEnd(
                    loop_end_positions
                        .iter()
                        .enumerate()
                        .filter(|(_, &end_pos)| end_pos == pos)
                        .map(|(id, _)| id)
                        .next()
                        .expect("no corresponding bracket found"),
                ));
            }
            _ => {}
        }
    }
    tokens
}

fn remove_comments(input: &str) -> String {
    input.chars().filter(|c| "><+-.,[]".contains(*c)).collect()
}
fn find_matching_close(s: &str, start: usize) -> usize {
    let mut level = 0;
    for (pos, c) in s.chars().enumerate().skip(start) {
        match c {
            '[' => level += 1,
            ']' => {
                level -= 1;
                if level == 0 {
                    return pos;
                }
            }
            _ => {}
        }
    }
    panic!("matching bracket close not found");
}

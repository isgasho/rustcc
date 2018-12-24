#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;
//use regex::Regex;


pub struct Token {
    pub name : String,
    pub value : String,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token { name: self.name.clone(), value: self.value.clone()}
    }
}

impl fmt::Display for Token {
    fn fmt (&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "name: {}, value: {}", self.name, self.value)
    }
}


pub fn is_number (c : char) -> bool {
   let nums = "0123456789";
   nums.contains(c)
}

pub fn is_letter (c : char) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters.contains(c)
}

pub fn is_punctuation (c : char) -> bool {
    let punc = "{}();";
    punc.contains(c)
}

pub fn read_identifier (input : &mut String) -> Token {
    let keywords = vec!["int", "return"];


    let mut iden_name = String::new();
    let tmp = input.clone();
    for c in tmp.chars() {
        if (!c.is_whitespace() && !is_punctuation(c)) {
            iden_name.push(c);
            input.remove(0);
        }
        else {
            break;
        }
    }

    if (keywords.contains(&&*iden_name)) {
        Token {name : String::from("Keyword"), value : iden_name}
    }
    else {
        Token {name : String::from("Identifier"), value : iden_name}
    }
}

pub fn read_number (input : &mut String) -> Token {
    let mut iden_name = String::new();
    let tmp = input.clone();
    for c in tmp.chars() {
        if (!c.is_whitespace() && !is_punctuation(c) && !is_letter(c)) {
            iden_name.push(c);
            input.remove(0);
        }
    }

    Token {name : String::from("Num"), value : iden_name}
}

pub fn read_punc (input : &mut String) -> Token {
    let ret_punc = (*input).chars().next().unwrap().to_string();
    input.remove(0); 
    Token {name : String::from("Punc"), value : ret_punc} 
}


pub fn lexer(input : &mut String) -> Vec<Token> {
    let mut token_vec : Vec<Token> = Vec::new();
    let mut c : char;

    while (input.len() > 0) {
        c = input.chars().next().unwrap();

        if (!c.is_whitespace()) {
            //println!("Character: {}", c);
            if (is_letter(c)) {
                // Must be identifier, as no quotes (not supported yet).
                token_vec.push(read_identifier(input));
            }
                else if (is_number(c)) {
                token_vec.push(read_number(input));
            }
            else if (is_punctuation(c)) {
                token_vec.push(read_punc(input));
            }
            else {
                println!("Found a character that the lexer does not recognize: {}.", c);       
                std::process::exit(1);
            }
        }
        else {
            input.remove(0);
        }
    }
    token_vec
}

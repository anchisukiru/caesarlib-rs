// Copyright 2017 Lukas A. Mueller.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

extern crate rand;

use rand::Rng;

trait StringChars<T> {
    fn all_chars(self) -> Vec<T>;
}

impl StringChars<String> for String {
    fn all_chars(self) -> Vec<String> {
        let char_vec: Vec<String> = self
                                    .split("")
                                    .map(|cur_char| { cur_char.to_string() })
                                    .collect();
        char_vec
    }
}

trait VecExtensions<T> {
    fn last_pos(&self) -> usize;
}

impl VecExtensions<String> for Vec<String> {
    fn last_pos(&self) -> usize {
        self.len() - 1
    }
}

fn vec_shift(input: &Vec<String>, start: usize, end: usize) -> String {
    input
    .iter()
    .enumerate()
    .filter(|&(i, _)| { i >= start && i <= end }) // end needed for one case otherwise not
    .map(|(_, x)| { x.to_string() })
    .collect::<Vec<String>>()
    .join("")
}

fn shift_seq(offset: u16, base_seq: String) -> String {
    let mut new_seq;
    let seq_vec: Vec<String> = base_seq.all_chars();

    if offset >= seq_vec.len() as u16 - 2 {
        println!("There!");
        let new_offset = offset as usize % (seq_vec.len() - 2);
        new_seq = vec_shift(&seq_vec, new_offset + 1, seq_vec.last_pos());
        if new_offset != 0 {
            new_seq = format!("{}{}", new_seq, vec_shift(&seq_vec, 0, offset as usize));
        }
    } else {
        println!("Here!");
        new_seq = vec_shift(&seq_vec, offset as usize + 1, seq_vec.last_pos());
        if offset != 0 {
            new_seq = format!("{}{}", new_seq, vec_shift(&seq_vec, 0, offset as usize));
        }
        println!("{}", new_seq);
    }
    new_seq
}

pub fn encipher(offset: u16, message: &str) -> String {
    let base = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base_seq = base.to_string().all_chars();
    let new_seq = shift_seq(offset, base.to_string()).all_chars();

    let mut new_string = String::new();

    for achar in message.to_string().all_chars() {
        let mut found = false;
        for i in 0..base_seq.last_pos() {
            if achar == base_seq[i] {
                new_string += &new_seq[i];
                found = true;
            }
        }
        if !found {
            new_string += &achar;
        }
    }

    new_string
}

pub fn decipher(offset: u16, message: &str) -> String {
    let base = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let cipher_seq = shift_seq(offset, base.to_string()).all_chars();
    let base_seq = base.to_string().all_chars();

    let mut new_string = String::new();

    for achar in message.to_string().all_chars() {
        let mut found = false;
        for i in 0..cipher_seq.last_pos() {
            if achar == cipher_seq[i] {
                new_string += &base_seq[i];
                found  = true;
            }
        }
        if !found {
            new_string += &achar;
        }
    }
    new_string
}

pub fn rdm_encipher(message: &str) -> (u16, String) {
    let mut rng = rand::thread_rng();
    let offset = rng.gen::<u16>();
    println!("{}", offset);
    (offset, encipher(offset, message))
}

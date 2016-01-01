use std::io;

use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn hanoi(n: u32, start: &str, temp: &str, target: &str, f: &mut File) -> io::Result<()> {
  //base case
  if n>1 {
    hanoi(n-1, start, target, temp, f);
  }
  try!(f.write_fmt(format_args!("Move disk {} from {} to {}.\n",n, start, target)));
  if n>1 {
    hanoi(n-1,temp,start,target,f);
  }
  
  Ok(())
}

fn main(){
  let mut f = File::create("output.txt").expect("Couldn't open file");
  let mut n = String::new();
  println!("How many disks are you trying to do? Ex: 12");
  io::stdin().read_line(&mut n).expect("Failed to read line");
  let n: u32 = n.trim().parse().expect("Please type a number!");
  let a: &str = "left";
  let b: &str = "middle";
  let c: &str = "right";
  hanoi(n,a,b,c,&mut f);
}


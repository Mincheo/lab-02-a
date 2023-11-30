use std::env;
use std::process;
fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() < 3{eprintln!("figure 1: too few arguments"); process::exit(1);}
    else if args.len() > 3{eprintln!("figure 1: too many arguments"); process::exit(1);}
    
    
    let str1  = &args[1];
    let str2  = &args[2];
    if let Err(_e) = str1.parse::<i32>() {eprintln!("figure 1: bad format"); process::exit(1);}
    else if let Err(_e) = str2.parse::<i32>() {eprintln!("figure 1: bad format"); process::exit(1);}
    let x : i32 = str1.parse().unwrap();
    let y : i32 = str2.parse().unwrap();
    
    if x>100 || y>100 {eprintln!("figure 1: out of range"); process::exit(1);}
    
    if x<=0 { 
    	if y>0{
    	     if(x>=-20 && y<=20) && (x*x+y*y) >=10*10{
    	     	if(x==-20 || y==20) || (x*x+y*y) ==10*10{
    	     	  println!("border");
    	     	}
    	     	else {println!("inside");}
    	     } else {println!("outside");}
    	} else {
    		if (x>=-20 && x <=-10) || (y>=-20 && y<=-10) {
    		  if (x==-20 || x ==-10) || (y==-20 || y==-10) {
    	     	  println!("border");
    	     	}
    	     	else {println!("inside");}
    	     } else {println!("outside");}
    	}
    }
    else {
    	if y>0 {
    		if (x*x+y*y) <= 20*20 && (x>=10 || y>=10) {
    		  if (x*x+y*y) == 20*20 || (x==10 || y==10) {
    	     	  println!("border");
    	     	}
    	     	else {println!("inside");}
    	     } else {println!("outside");}
    	}else {
    	  if ((x*x+y*y)>=10*10) && ((x*x+y*y)<=20*20) {
    	    if ((x*x+y*y)==10*10) || ((x*x+y*y)==20*20) {
    	     	  println!("border");
    	     	}
    	     	else {println!("inside");}
    	     } else {println!("outside");}
    	
    	}
    	
    }
}

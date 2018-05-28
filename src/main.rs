extern crate colored;

use colored::*;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

mod ascii;

fn main() {
    welcome();
    hostname();
    println!("\n");
    disk_list();
    up_time();
}

fn disk_list() {
    let output = Command::new("df")
    			  .arg("/").arg("/boot").arg("--output=source,size,used,avail,pcent,target")
			  .arg("-h")
			  .output()
			  .expect("Failed to execute process!");
    
    let command_output = String::from_utf8_lossy(&output.stdout);

    let mut output_list = command_output.split("\n");

    let mut final_output: String = String::from("");
    
    match output_list.next() {
        Some(i) => final_output.push_str(i), 
	_ => (),
    }

    let header_length = final_output.len() as u32;

    final_output.push_str("\n");

    loop {
        match output_list.next() {
	    None => break,
	    Some(i) => 
	        {
		    final_output.push_str(i);
		    final_output.push_str("\n");

		    let mut df_percent = i.split_whitespace().nth(4);
		    let mut df_stringy: String;

		    match df_percent {
		        Some(i) => {df_stringy = i.to_string()},
			None => break,
		    }
		    
		    df_stringy.pop();

		    let mut df_numeric: u32;
		    match df_stringy.parse::<u32>() {
		        Ok(t) => df_numeric = t,
			_ => break,
		    }

		    final_output.push_str(&bars(&header_length, &df_numeric));
		    final_output.push_str("\n");
		},
		
	}
    }

    println!("{}", final_output);
}

fn hostname() {
    let output = Command::new("hostname").output().expect("Failed to execute process!");
    let string_output = ascii::build(String::from_utf8_lossy(&output.stdout).to_string());
    println!("{}", string_output.red());
}

fn welcome() {
    let output = Command::new("uname")
    			  .arg("-sr")
			  .output()
			  .expect("Failed to execute process!");
    let string_output = String::from_utf8_lossy(&output.stdout);
    let mut print_out = String::from("Welcome to ");

    let etc_release_result = read_os_file(String::from("/etc/os-release"));
    let etc_release: String;
    match etc_release_result {
        Ok(i) => etc_release = i,
	_ => etc_release = String::from("PRETTY_NAME=\"Unknown\""),
    }

    let mut release = etc_release.split("\n");

    loop {
	match release.next() {
	    None => break,
	    Some(i) => {
			    if i.starts_with("PRETTY_NAME=") {
				let mut s = i.replace("\"","");
				s = s.replace("PRETTY_NAME=", "");
				print_out.push_str(&s);
			    }
	    		},
	}
    }

    print_out.push_str("! (");
    print_out.push_str(&string_output.trim());
    print_out.push_str(")");
    println!("{}", print_out.red().bold());
}

fn bars(width: &u32, pcent: &u32) -> String {
    let bar_length = ((*width as f32 - 2f32) * (*pcent as f32 / 100f32)) as u32;
    let mut output = String::from("[");

    for i in 1..width - 1 {
        if i < bar_length {
	    output.push_str("|");
	} else {
	    output.push_str(" ");
	}
    }

    output.push_str("]");

    return output;
}

fn read_os_file(file_name: String) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn up_time() {
    let output = Command::new("uptime").arg("-p").output().expect("Failed to execute process!");
    println!("{}", String::from_utf8_lossy(&output.stdout).underline());
}

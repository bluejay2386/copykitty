mod mapping;

pub fn build(input: String) -> String {
    let char_map = mapping::load_map();
    let mut output = (String::from("    "), String::from("   "), String::from("  "), String::from(" "), String::from(""));

    for i in input.chars() {
	match char_map.get(&i) {
	    Some(j) => {
	    	output = concat(output, j);
	    },
	    _ => (),
	}
    }

    let mut final_out = String::from("");
    final_out.push_str(&output.0);
    final_out.push_str("\n");
    final_out.push_str(&output.1);
    final_out.push_str("\n");
    final_out.push_str(&output.2);
    final_out.push_str("\n");
    final_out.push_str(&output.3);
    final_out.push_str("\n");
    final_out.push_str(&output.4);

    final_out
}

fn concat(origin: (String, String, String, String, String), to_add: &(String, String, String, String, String)) -> (String, String, String ,String, String) {
    let (mut a, mut b, mut c, mut d, mut e) = origin;
    let poppable = [is_replaceable(&a, &to_add.0), is_replaceable(&b, &to_add.1), is_replaceable(&c, &to_add.2), is_replaceable(&d, &to_add.3), is_replaceable(&e, &to_add.4)];

    let mut cant_pop = false;

    for i in poppable.iter(){
        if *i == 0 {
	    cant_pop = true;
	}
    }

    if cant_pop {
        a.push_str(&to_add.0.clone());
        b.push_str(&to_add.1.clone());
        c.push_str(&to_add.2.clone());
        d.push_str(&to_add.3.clone());
        e.push_str(&to_add.4.clone());
        return (a, b, c, d, e);
    }

    if poppable[0] == 1 {
        a.pop();
	a.push_str(&to_add.0.clone());
    } else {
	a.push_str(&to_add.0[1..]);
    }

    if poppable[1] == 1 {
	b.pop();
	b.push_str(&to_add.1.clone());
    } else {
        b.push_str(&to_add.1[1..])
    }

    if poppable[2] == 1 {
        c.pop();
	c.push_str(&to_add.2.clone());
    } else {
        c.push_str(&to_add.2[1..])
    }

    if poppable[3] == 1 {
        d.pop();
	d.push_str(&to_add.3.clone());
    } else {
        d.push_str(&to_add.3[1..])
    }

    if poppable[4] == 1 {
        e.pop();
	e.push_str(&to_add.4.clone());
    } else {
        e.push_str(&to_add.4[1..]);
    }
    (a, b, c, d, e)
}

fn is_replaceable(input1: &str, input2: &str) -> u32 {

    if input2.starts_with(" "){
	return 2;
    }

    if input1.ends_with(" ") || input1.ends_with("/") || input1.ends_with("|") {
	return 1;
    }

    match input2.get(..1) {
	Some(i) => {
	    if input1.ends_with(i) {
		return 1;
	    }
	},
        _ => (),
    }

    return 0;
}

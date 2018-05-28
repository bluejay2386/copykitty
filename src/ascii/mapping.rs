use std::collections::HashMap;


pub fn load_map() -> HashMap<char, (String, String, String, String, String)> {
    let mut alpha_map = HashMap::new();

alpha_map.insert('a', (String::from("___     "), String::from("/   |   "), String::from("/ /| |  "), String::from("/ ___ | "), String::from("/_/  |_|")));
alpha_map.insert('b', (String::from("____   "), String::from("/ __ ) "), String::from("/ __  |"), String::from("/ /_/ /"), String::from("/_____/")));
alpha_map.insert('c', (String::from("______ "), String::from("/ ____/"), String::from("/ /    "), String::from("/ /___ "), String::from(" \\____/")));
alpha_map.insert('d', (String::from("____   "), String::from("/ __ \\ "), String::from("/ / / /"), String::from("/ /_/ /"), String::from("/_____/")));
alpha_map.insert('e', (String::from("______ "), String::from("/ ____/"), String::from("/ __/  "), String::from("/ /___ "), String::from("/_____/")));
alpha_map.insert('f', (String::from("______ "), String::from("/ ____/"), String::from("/ /_   "), String::from("/ __/  "), String::from("/_/    ")));
alpha_map.insert('g', (String::from("______ "), String::from("/ ____/"), String::from("/ / __ "), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('h', (String::from("__  __ "), String::from("/ / / /"), String::from("/ /_/ /"), String::from("/ __  /"), String::from("/_/ /_/")));
alpha_map.insert('i', (String::from("____ "), String::from("/  _/"), String::from(" / / "), String::from("_/ / "), String::from("/___/")));
alpha_map.insert('j', (String::from("    __ "), String::from("    / /"), String::from("__  / /"), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('k', (String::from("__ __  "), String::from("/ //_/ "), String::from("/ ,<   "), String::from("/ /| | "), String::from("/_/ |_|")));
alpha_map.insert('l', (String::from("__     "), String::from("/ /    "), String::from("/ /    "), String::from("/ /___ "), String::from("/_____/")));
alpha_map.insert('m', (String::from("__  ___ "), String::from("/  |/  /"), String::from("/ /|_/ /"), String::from("/ /  / /"), String::from("/_/  /_/")));
alpha_map.insert('n', (String::from("_   __ "), String::from("/ | / /"), String::from("/  |/ /"), String::from("/ /|  /"), String::from("/_/ |_/")));
alpha_map.insert('o', (String::from("____   "), String::from("/ __ \\ "), String::from("/ / / /"), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('p', (String::from("____   "), String::from("/ __ \\ "), String::from("/ /_/ /"), String::from("/ ____/"), String::from("/_/    ")));
alpha_map.insert('q', (String::from("____    "), String::from("/ __ \\  "), String::from("/ / / / "), String::from("/ /_/ / "), String::from(" \\___\\_\\")));
alpha_map.insert('r', (String::from("____   "), String::from("/ __ \\ "), String::from("/ /_/ /"), String::from("/ _, _/"), String::from("/_/ |_|")));
alpha_map.insert('s', (String::from("_____  "), String::from("/ ___/ "), String::from(" \\__ \\ "), String::from(" ___/ /"), String::from(" /____/")));
alpha_map.insert('t', (String::from("______ "), String::from("/_  __/"), String::from("  / /  "), String::from("  / /  "), String::from("  /_/  ")));
alpha_map.insert('u', (String::from("__  __ "), String::from("/ / / /"), String::from("/ / / /"), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('v', (String::from("_    __ "), String::from("| |  / /"), String::from(" | | / /"), String::from("  | |/ /"), String::from("   |___/")));
alpha_map.insert('w', (String::from("_       __ "), String::from("| |     / /"), String::from(" | | /| / /"), String::from("  | |/ |/ /"), String::from("   |__/|__/")));
alpha_map.insert('x', (String::from("_  __  "), String::from("| |/ / "), String::from(" |   / "), String::from(" /   | "), String::from(" /_/|_|")));
alpha_map.insert('y', (String::from("__  __ "), String::from(" \\ \\/ /"), String::from("   \\  /"), String::from("    / /"), String::from("    /_/")));
alpha_map.insert('z', (String::from("_____    "), String::from("/__  /   "), String::from("   / /   "), String::from("   / /__ "), String::from("   /____/")));
alpha_map.insert('-', (String::from("       "), String::from("       "), String::from("______ "), String::from("/_____/"), String::from("       ")));
alpha_map.insert('1', (String::from("___ "), String::from("<  /"), String::from(" / /"), String::from(" / /"), String::from(" /_/")));
alpha_map.insert('2', (String::from("___    "), String::from("|__ \\  "), String::from(" __/ / "), String::from(" / __/ "), String::from(" /____/")));
alpha_map.insert('3', (String::from("_____  "), String::from("|__  / "), String::from("  /_ < "), String::from(" ___/ /"), String::from(" /____/")));
alpha_map.insert('4', (String::from("__ __   "), String::from("/ // /  "), String::from("/ // /_ "), String::from("/__  __/"), String::from("   /_/  ")));
alpha_map.insert('5', (String::from("______ "), String::from("/ ____/"), String::from("/___ \\ "), String::from("____/ /"), String::from("/_____/")));
alpha_map.insert('6', (String::from("_____  "), String::from("/ ___/ "), String::from("/ __ \\ "), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('7', (String::from("_____ "), String::from("/__  /"), String::from("   / /"), String::from("   / /"), String::from("   /_/")));
alpha_map.insert('8', (String::from("____   "), String::from("( __ ) "), String::from("/ __  |"), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map.insert('9', (String::from("____   "), String::from("/ __ \\ "), String::from("/ /_/ /"), String::from(" \\__, /"), String::from(" /____/")));
alpha_map.insert('0', (String::from("____ "), String::from("/ __ \\"), String::from("/ / / /"), String::from("/ /_/ /"), String::from(" \\____/")));
alpha_map
}

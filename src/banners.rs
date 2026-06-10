pub fn get_banners(name: &str) -> Vec<&'static str> {
    match name {
        "batman" => vec![
        "           |\\_|\\               ",
        "           | a_a\\              ",
        "           | | \"]              ",
        "       ____| '-\\___            ",
        "      /.----.___.-'\\           ",
        "     //        _    \\          ",
        "    //   .-. (~v~) /|          ",
        "   |'|  /\\:  .--  / \\          ",
        "  // |-/  \\_/____/\\/~|        ",
        " |/  \\ |  []_|_|_] \\ |        ",
        " | \\  | \\ |___   _\\ ]_}      ",
        " | |  '-' /   '.'  |           ",
        " | |     /    /|:  |           ",
        " | |     |   / |:  /\\         ",
        " | |     /  /  |  /  \\        ",
        " | |    |  /  /  |    \\       ",
        " \\ |    |/\\/  |/|/\\    \\   ",
        "  \\|\\ |\\|  |  | / /\\/\\__\\",
        "   \\ \\| | /   | |__          ",
        "        / |   |____)           ",
        "        |_/                    ",
        ],

        "dog" => vec![
                        "   / \\__        ",
            "  (    @\\___    ",
            "  /         O   ",
            " /   (_____/    ",
            "/_____/   U     ",
        ],
        // standart banner is batman
        _ => get_banners("batman"),
    }
}

// Returns a list of all available banner names
pub fn list_banners() -> Vec<&'static str> {
    vec!["batman", "dog"]
}
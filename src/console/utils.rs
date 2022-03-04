use rand::seq::SliceRandom;

pub fn welcome_lines() -> Vec<String> {
    let lines: Vec<&str> = vec![
        "// -------------------------------------------------------------------------- //",
        "//                                                                            //",
        "//                              BOZOS ver 3.0.2                               //",
        "//                                                                            //",
        "//                          Compute with confidence.                          //",
        "// -------------------------------------------------------------------------- //",
        " ",
        "+ SYSTEM STATE --------------------------------------------------------------  +",
        "> System up to date and operational",
        " ",
        "+ HOW TO USE ----------------------------------------------------------------  +",
        "To interact with this terminal, type commands in the input line below.",
        "Then, execute the command using the <Return> key on your (real) keyboard.",
        "If you are lost, enter 'help' to show this message again.",
        " ",
        "+ AVAILABLE COMMANDS --------------------------------------------------------  +",
        "    - help: show the available commands",
        "    - cheat <code>: enable a cheat code to activate an ability",
        "    - log: display a log entry",
        "    - clear: clear the entire display",
        "    - exit: exit this terminal to go back to boring reality",
        " ",
        " ",
    ];

    lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

pub fn display_help() -> String {
    let lines: Vec<&str> = vec![
        "+ HOW TO USE ----------------------------------------------------------------  +",
        "To interact with this terminal, type commands in the input line below.",
        "Then, execute the command using the <Return> key on your (real) keyboard.",
        "If you are lost, enter 'help' to show this message again.",
        " ",
        "+ AVAILABLE COMMANDS --------------------------------------------------------  +",
        "    - help: show the available commands",
        "    - cheat <code>: enable a cheat code for this world",
        "    - log: display a log entry",
        "    - clear: clear the entire display",
        "    - exit: exit this terminal to go back to boring reality",
        " ",
    ];

    lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn display_random_log() -> String {
    let logs: Vec<Vec<&str>> = vec![
        vec![
            "3/4/2077:",
            " ",
            "To keep us safe, the world government has banned leftward movement.",
            " ",
            "President Bozo: \"It is unnecessary and dangerous for the average citizen to have",
            "the ability to move left. If people cannot move left, they can only move forward",
            "and do what is right!\"",
        ],
        vec![
            "2/25/2077:",
            " ",
            "Grandpa told me the strangest thing yesterday. He said that our NFT animals",
            "used to actually exist outside the Bozoverse! I wonder what it would feel like to",
            "pet FunkyKitty#5632 in the real world :)",
        ],
        vec![
            "12/11/2076:",
            " ",
            "To keep us safe, the world government has banned jumping.",
            " ",
            "President Bozo: No ordinary citizen needs the ability to jump.",
            "However, those that can pass a written test and pay the fee will",
            "be granted the ability to jump with a jumping license. Reducing",
            "access to jumping will reduce crime on our streets.",
        ],
    ];

    let random_log = logs.choose(&mut rand::thread_rng()).unwrap();
    random_log
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

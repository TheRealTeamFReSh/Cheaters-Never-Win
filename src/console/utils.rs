pub fn welcome_lines() -> Vec<String> {
    let lines: Vec<&str> = vec![
        "// -------------------------------------------------------------------------- //",
        "//              *******                      *******    ********              //",
        "//              /**////**                    **/////**  **//////              //",
        "//              /**    /**  *****  **    ** **     //**/**                    //",
        "//              /**    /** **///**/**   /**/**      /**/*********             //",
        "//              /**    /**/*******//** /** /**      /**////////**             //",
        "//              /**    ** /**////  //****  //**     **        /**             //",
        "//              /*******  //******  //**    //*******   ********              //",
        "//              ///////    //////    //      ///////   ////////               //",
        "//                                ver 3.0.2                                   //",
        "//                                                                            //",
        "//                        Welcome into an unfair world                        //",
        "// -------------------------------------------------------------------------- //",
        " ",
        "+ SYSTEM STATE --------------------------------------------------------------  +",
        "Angry bubbler: 0 / 10 online",
        "Cheat codes activated: 1 / 12",
        ">System< up to date and operational",
        " ",
        "+ HOW TO USE ----------------------------------------------------------------  +",
        "Type commands in the input line below in order to interract with this terminal,",
        "then send the command using the <Return> key on your (real) keyboard.",
        "If you are lost, you can still type 'help' to display indications and",
        "suggestions.",
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
        "Type commands in the input line below in order to interract with this terminal,",
        "then send the command using the <Return> key on your (real) keyboard.",
        "If you are lost, you can still type 'help' to display indications and",
        "suggestions.",
        " ",
        "+ AVAILABLE COMMANDS --------------------------------------------------------  +",
        "    - help: show the available commands",
        "    - use <code>: enable a cheat code for this world",
        "    - exit: exit this terminal to go back to the boring reality",
        " ",
    ];

    lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

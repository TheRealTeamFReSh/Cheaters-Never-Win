pub fn welcome_lines() -> Vec<String> {
    let mut lines: Vec<&str> = Vec::new();

    lines.push("// -------------------------------------------------------------------------- //");
    lines.push("//              *******                      *******    ********              //");
    lines.push("//              /**////**                    **/////**  **//////              //");
    lines.push("//              /**    /**  *****  **    ** **     //**/**                    //");
    lines.push("//              /**    /** **///**/**   /**/**      /**/*********             //");
    lines.push("//              /**    /**/*******//** /** /**      /**////////**             //");
    lines.push("//              /**    ** /**////  //****  //**     **        /**             //");
    lines.push("//              /*******  //******  //**    //*******   ********              //");
    lines.push("//              ///////    //////    //      ///////   ////////               //");
    lines.push("//                                ver 3.0.2                                   //");
    lines.push("//                                                                            //");
    lines.push("//                        Welcome into an unfair world                        //");
    lines.push("// -------------------------------------------------------------------------- //");
    lines.push(" ");

    lines.push("+ HOW TO USE ----------------------------------------------------------------  +");
    lines.push("Type commands in the input line below in order to interract with this terminal,");
    lines.push("then send the command using the <Return> key on your (real) keyboard.");
    lines.push("If you are lost, you can still type 'help' to display indications and");
    lines.push("suggestions.");
    lines.push(" ");

    lines.push("+ COMMANDS ------------------------------------------------------------------  +");
    lines.push("    - help: show the available commands");
    lines.push("    - use <code>: enable a cheat code for this world");
    lines.push("    - exit: exit this terminal to go back to the boring reality");
    lines.push(" ");

    lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

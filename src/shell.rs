use std::io::{self, Write};
use crate::commands;
pub fn run() {
    clear_screen();
    print_banner();

    println!("\n📝 Type `help` to see available commands.\n");

    loop {
        print!("geekpad > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("⚠️ Failed to read input");
            continue;
        }

        let command = input.trim();
        let mut parts = command.splitn(2, ' ');
        let main = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();

        match main {
            "help" => {
                println!("\
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🛠️  Available Main Commands:
   • new        → Create a new note
   • view       → View an existing note
   • rm         → Delete a note
   • ls         → List all saved notes
   • edit       → Edit an existing note
   • clear      → Clear the screen
   • exit       → Exit Geekpad
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎯 Slash Commands (use on a new line):
   • /alldone         → Save and exit the editor
   • /bold            → Apply bold styling
   • /italic          → Apply italic/cursive styling
   • /hr              → Insert horizontal rule
   • /reset           → Restore default styles
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                ");
            }

            "new" => {
                if arg.is_empty() {
                    println!("⚠️ Usage: new <note-name>");
                } else {
                    commands::create_note(arg);
                }
            }

            "view" => {
                if arg.is_empty() {
                    println!("⚠️ Usage: view <note-name>");
                } else {
                    commands::view_note(arg);
                }
            }

            "rm" => {
                if arg.is_empty() {
                    println!("⚠️ Usage: rm <note-name>");
                } else {
                    commands::delete_note(arg);
                }
            }

            "edit" => {
                if arg.is_empty() {
                    println!("⚠️ Usage: edit <note-name>");
                } else {
                    commands::edit_note(arg);
                }
            }

            "ls" => {
                commands::list_notes();
            }

            "theme" => {
                commands::change_theme();
            }

            "clear" => {
                clear_screen();
                cleared_banner();
            }

            "exit" | "quit" => {
                println!("👋 Exiting Geekpad. See you soon!");
                break;
            }

            "" => continue,

            _ => println!("❌ Unknown command: '{}'", command),
        }
    }
}


fn print_banner() {
    println!(
        r#"
────────────────────────────────────────────────────────────────────────
                        🌟 Welcome to Geekpad 🧠                    
              A Secure CLI Notepad for Geeks & Creators           
────────────────────────────────────────────────────────────────────────
 🔐  Encrypted with AES-GCM      
 ⚙️  Built in Rust                      
 ❤️  Made with love in Melbourne  
 
 ✨  My first real Rust project — built to apply, not just learn             
     and to pave the way for a future powered by meaningful tools. 
     This is for people who love CLI. From a guy who thought CLI apps
     are a joke to falling in love with it.                                       
                                                                        
 👨‍💻  Developer: Eswar                                        
 🌐  GitHub   : https://github.com/Eswar-S-Sethu                        
 💼  LinkedIn : https://www.linkedin.com/in/eswar-sivan-sethu-781294228 
─────────────────────────────────────────────────────────────────────────
"#
    );
}

fn cleared_banner(){
    println!(
        r#"
────────────────────────────────────────────────────────────────────────
                        🌟 Welcome to Geekpad 🧠                    
              A Secure CLI Notepad for Geeks & Creators           
────────────────────────────────────────────────────────────────────────
 🔐  Encrypted with AES-GCM      
 ⚙️  Built in Rust                      
 ❤️  Made with love in Melbourne   
─────────────────────────────────────────────────────────────────────────
"#
    );
}


fn clear_screen() {
    // optional screen clear
    print!("\x1B[2J\x1B[1;1H");
}

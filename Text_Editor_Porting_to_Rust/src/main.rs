use crossterm::{
    event::{read, KeyCode,Event::*},
    Result,
    terminal
    };


fn main() -> Result<()>{
    
    terminal::enable_raw_mode();
    loop {
        if let Ok(event) = read()  {
            
            // when we press q we'll exit
            if let Key(key_event) = event {
                if key_event.code == KeyCode::Char('q')
                {
                    break;
                }
                else
                {
                    // what is the question after read for ?
                    println!("{:?}\r", key_event);
                }
            }
            
            
        }else {
            break;
        }
    }
    terminal::disable_raw_mode();
    Ok(())
}

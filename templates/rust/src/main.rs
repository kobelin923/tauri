mod cmd;

extern crate tauri;

#[macro_use]
extern crate serde_derive;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(_) => {}
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
        }
      }
    })
    .build()
    .run();
}

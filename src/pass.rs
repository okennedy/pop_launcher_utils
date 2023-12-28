
mod launcher;
mod pass_lib;
mod pass_menu;


use std::io;
use pass_menu::Menu;
use launcher::{ Request, PluginResponse };
use std::result::Result;

fn respond(response: &PluginResponse) -> io::Result<()>
{
  let encoded = serde_json::to_string(&response)?;
  print!("{}\n", encoded);
  Ok(())
}

fn process_request(request_str: &String, menu: &Menu) -> Result<bool, String> 
{
  let request: Request = serde_json::from_str(&request_str).map_err(|err| err.to_string())?;

  match request {
    Request::Activate(id) => {
      respond(&PluginResponse::Close).map_err(|err| err.to_string())?;
      menu.activate(id)?;
    }
    Request::ActivateContext { id, context } => 
      menu.activate_context(id, context)?,
    Request::Complete(_id) => 
      (),
    Request::Context(id) => 
      respond(
        &PluginResponse::Context { 
          id: id, 
          options: menu.context(id) 
        }
      ).map_err(|err| err.to_string())?,
    Request::Exit => {
      return Ok(false);
    } 
    Request::Interrupt => 
      (),
    Request::Quit(_id) => 
      (),
    Request::Search(term) => {
      for entry in menu.search(term.trim_start_matches("pass "))
      {
        respond(&PluginResponse::Append(entry)).map_err(|err| err.to_string())?
      }
      respond(&PluginResponse::Finished).map_err(|err| err.to_string())?
    }
  }

  Ok(true)
}


fn main() -> Result<(), String> {
  let mut keep_going = true;

  let mut buffer = String::new();
  let stdin = io::stdin();

  let passwords = pass_lib::ls().map_err(|err| err.to_string())?;
  let menu = Menu::build(passwords);

  while keep_going {
    if stdin.read_line(&mut buffer).map_err(|err| err.to_string())? > 0
    {
      keep_going = process_request(&buffer, &menu).map_err(|err| err.to_string())?;
      buffer.clear();
    } else {
      keep_going = false;
    }
  }


  Ok(())
}

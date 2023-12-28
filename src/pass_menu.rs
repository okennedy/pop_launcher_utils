use std::vec::Vec;
use crate::pass_lib::PassList;
use crate::launcher::{self, PluginSearchResult};
use std::process::Command;
use num_derive::FromPrimitive;

pub struct MenuItem
{
  plugin_entry: launcher::PluginSearchResult,
  full_name: String
}

pub struct Menu
{
  entries: Vec<MenuItem>
} 

#[derive(FromPrimitive)]
enum ContextAction
{
  Copy = 1,
  Edit,
}

impl Menu
{
  fn add_all(&mut self, entries: PassList, prefix: &str, id: &mut u16)
  {
    for entry in entries
    {

      let name = entry.name.clone();
      let mut full_name = String::from(prefix);
      // full_name.push('/');
      full_name.push_str(name.as_str());

      match entry.children
      {
        None => {
          let menu_item = 
            MenuItem {
              plugin_entry: 
                PluginSearchResult {
                  id: u32::from(*id),
                  name: name.clone(),
                  description: full_name.clone(),
                  keywords: None,
                  icon: Some(launcher::IconSource::Name(String::from("security-high"))),
                  exec: None,
                  window: None
                },
              full_name: full_name.clone()
            };          
            assert_eq!(self.entries.len(), usize::from(*id));
            self.entries.push(menu_item);
            *id = *id + 1;
        }
        Some(children) =>
        {
          self.add_all(children, (full_name+"/").as_str(), id)
        }
      }


    }
  }

  pub fn build(entries: PassList) -> Menu
  {
    let mut menu = Menu {
      entries: Vec::new(),
    };
    let mut id = 0;
    menu.add_all(entries, "", &mut id);

    menu
  }

  pub fn search(&self, term: &str) -> Vec<launcher::PluginSearchResult>
  {
    let mut ret = Vec::new();

    ret.extend(
      self.entries
          .iter()
          .filter( |entry| {
            // print!("Check: {}", entry.full_name);
            entry.full_name.contains(term)
          }
          )
          .map( |entry|
            entry.plugin_entry.clone()
          )
    );

    ret
  } 

  pub fn activate(&self, id: u32) -> Result<(),String>
  {
    let idx = usize::try_from(id)
                    .map_err(|_err| "Invalid index")?;
    if idx >= self.entries.len() { return Err(String::from("Invalid index")) }
    let entry = &self.entries[idx];

    Command::new("pass")
            .arg("-c")
            .arg(entry.full_name.as_str())
            .spawn()
            .map_err(|err| err.to_string())?;

    Ok(())
  }

  pub fn edit(&self, id: u32) -> Result<(),String>
  {
    let idx = usize::try_from(id)
                    .map_err(|_err| "Invalid index")?;
    if idx >= self.entries.len() { return Err(String::from("Invalid index")) }
    let entry = &self.entries[idx];

    Command::new("pass")
            .arg("edit")
            .arg(entry.full_name.as_str())
            .spawn()
            .map_err(|err| err.to_string())?;

    Ok(())
  }

  pub fn context(&self, _id: u32) -> Vec<launcher::ContextOption>
  {
    let mut ret:Vec<launcher::ContextOption> = Vec::new();
    ret.push(launcher::ContextOption{
      id: ContextAction::Copy as u32,
      name: String::from("Copy")
    });
    ret.push(launcher::ContextOption{
      id: ContextAction::Edit as u32,
      name: String::from("Edit")
    });
    return ret
  }


  pub fn activate_context(&self, id: u32, action: u32) -> Result<(), String>
  {
    match num::FromPrimitive::from_u32(action)
    {
      Some(ContextAction::Copy) => self.activate(id)?,
      Some(ContextAction::Edit) => self.edit(id)?,
      None => return Err(String::from("Invalid context action"))
    }

    Ok(())
  }
}


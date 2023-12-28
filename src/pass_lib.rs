
use std::vec::Vec;
use std::option::Option;
use std::fs;
use std::io;
use std::path::Path;
use std::env;

pub type PassList = Vec<PassEntry>;

pub struct PassEntry
{
  pub name: String,
  pub children: Option<PassList>
}

fn list_dir(dir: &Path) -> io::Result<PassList>
{
  let mut list:PassList = Vec::new();

  for entry in fs::read_dir(dir)?
  {
    let entry = entry?.path();
    if entry.is_dir() 
    {
      match entry.file_name()
                 .and_then( |x| x.to_str() )
      {
        None => (),
        Some(filename) =>
          list.push(
            PassEntry { 
              name: String::from(filename),
              children: Some(list_dir(&entry.as_path())?)
            }
          )

      }      
    } else if entry.extension()
                   .and_then( |x| x.to_str() )
                   .unwrap_or(&"nogo") == "gpg"
    {
      match entry.file_stem()
                 .and_then( |x| x.to_str() )
      {
        None => (),
        Some(filename) => 
          list.push(
            PassEntry { 
              name: String::from(filename),
              children: None
            }
          )

      }
    }

  }

  return Ok(list)
}

pub fn ls() -> io::Result<PassList>
{
  // home_dir has a deprecation warning because it is broken on
  // windows... but PopOS is linux only. 
  #[allow(deprecated)]
  let mut dir = 
    match env::home_dir() {
      Some(dir) => dir,
      None => return Err(io::Error::new(io::ErrorKind::NotFound, "Can't find Password Directory (no home directory)"))
    };

  dir.push(".password-store");

  Ok(list_dir(&dir.as_path())?)
}

mod launcher;
#[cfg(test)] mod todo_tests;

use chrono::Local;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use launcher::{ Request, PluginResponse, PluginSearchResult, IconSource };
use std::io;
use std::error;
use date_time_parser::DateParser;
use reqwest::blocking::Client;
use uuid::Uuid;

use std::result;
use std::env;
use std::fs;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

struct Todo
{
  summary: String,
  due: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config
{
  url: String,
  username: String,
  password: String,
  calendars: Vec<String>
}

struct Session
{
  todo: Todo,
  config: Config,
}

fn respond(response: &PluginResponse) -> Result<()>
{
  let encoded = serde_json::to_string(&response)?;
  print!("{}\n", encoded);
  Ok(())
}

fn task_string(session: &Session) -> String
{
  format!("{}{}", 
    session.todo.summary,
    session.todo.due.map(|d| " ".to_owned() + &d.to_string()).unwrap_or("".to_string()), 
  )
}

fn build_menu(session: &Session) -> Result<()>
{
  let task = task_string(session);
  for (idx, cal) in session.config.calendars.iter().enumerate()
  {
    respond(&PluginResponse::Append(
      PluginSearchResult {
        id: idx as u32,
        name: "Create in ".to_owned()+cal.as_str(),
        description: task.clone(),
        keywords: None,
        icon: Some(IconSource::Name("appointment-new".to_string())),
        exec: None,
        window: None
      }
    ))?;
  }
  respond(&PluginResponse::Finished)?;
  Ok(())  
}

fn parse_task(sentence: &String, session: &mut Session) -> Result<()>
{
  let mut summary = String::new();
  let mut date:Option<NaiveDate> = None;
  for term in sentence.strip_prefix("todo").unwrap_or(sentence).split(" ")
  {
    if term.is_empty()
    {
      ()
    }
    else if term.starts_with("@")
    {
      if let Some(parsed) = term.get(1..).and_then(|d| { DateParser::parse(d.replace("_"," ").as_str()) })
      {
        date = Some(parsed)
      } else {
        if !summary.is_empty() { summary.push(' '); }
        summary.push_str(term)
      }
    }
    else 
    {
      if !summary.is_empty() { summary.push(' '); }
      summary.push_str(term)
    }
  }
  session.todo.summary = summary;
  session.todo.due = date;
  Ok(())
}

fn gen_caldav(todo: &Todo, uuid: Uuid) -> String
{
  let due = match &todo.due 
            {
              None => "".to_string(),
              Some(due) => format!("DUE:{}\n", due.format("%Y%m%dT190000").to_string())
            };

  let mut caldav = String::new();
  caldav.push_str("BEGIN:VCALENDAR\n");
  caldav.push_str("PRODID:-//okennedy//pop_todo//EN\n");
  caldav.push_str("VERSION:2.0\n");
  caldav.push_str("BEGIN:VTODO\n");
  caldav.push_str(format!("UID:{}\n", uuid).as_str());
  caldav.push_str(format!("DTSTAMP:{}\n", Local::now().format("%Y%m%dT%H%M%S").to_string()).as_str());
  caldav.push_str(due.as_str());
  caldav.push_str(format!("SUMMARY:{}\n", todo.summary).as_str());
  caldav.push_str("END:VTODO\n");
  caldav.push_str("END:VCALENDAR\n");

  return caldav  
}

fn publish_todo(todo: &Todo, base_url: &String, username: &String, password: &String, calendar: &String) -> Result<()>
{
  let client = Client::new();
  let uuid = Uuid::new_v4();

  let url = format!("{}/calendars/{}/{}/{}.ics", base_url, username, calendar, uuid);
  let caldav = gen_caldav(todo, uuid);

  println!("{}", url);

  let resp = 
    client.put(url)
          .body(caldav)
          .basic_auth(username, Some(password))
          .header("Content-Type", "text/calendar")
          .send()?;

  println!("Status: {}\n{}", resp.status(), resp.text()?);

  Ok(())
}

fn process_request(request: &Request, session: &mut Session) -> Result<bool> 
{
  match request {
    Request::Activate(id) => {
      respond(&PluginResponse::Clear)?;
      publish_todo(
        &session.todo,
        &session.config.url, 
        &session.config.username, 
        &session.config.password,
        &session.config.calendars[*id as usize]
      )?;
      respond(&PluginResponse::Close)?;
    }
    Request::Exit => {
      return Ok(false);
    } 
    Request::Search(term) => {
      parse_task(term, session)?;
      build_menu(session)?;
    }
    Request::ActivateContext { id:_id, context:_context } => (),
    Request::Complete(_) => (),
    Request::Context(_) => (),
    Request::Interrupt => (),
    Request::Quit(_) => (),
  }

  Ok(true)
}


fn main() -> Result<()> {

  // home_dir has a deprecation warning because it is broken on
  // windows... but PopOS is linux only. 
  #[allow(deprecated)]

  let mut config = env::home_dir().unwrap();
  config.push(".config");
  config.push("pop_todo.json");
  let config_string = fs::read_to_string(config)?;
  let config = serde_json::from_str(config_string.as_str())?;

  let mut session = Session {
    todo: Todo {
      summary: "No Description".to_string(),
      due: None,
    },
    config,
  };

  let mut keep_going = true;

  let mut buffer = String::new();
  let stdin = io::stdin();


  while keep_going {
    if stdin.read_line(&mut buffer)? > 0
    {
      let request: Request = serde_json::from_str(&buffer)?;
      keep_going = process_request(&request, &mut session)?;
      buffer.clear();
    } else {
      keep_going = false;
    }
  }


  Ok(())
}

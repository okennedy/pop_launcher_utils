use date_time_parser::DateParser;
use chrono::{ Local, Datelike, Duration };
use chrono::naive::NaiveDate;
use uuid::Uuid;

use crate::{Session, Todo, parse_task, gen_caldav};


#[test]
fn test_date_creation() 
{
    let date1 = DateParser::parse("today");
    let date1cmp = Local::now();
    assert_eq!(NaiveDate::from_ymd_opt(date1cmp.year(), date1cmp.month(), date1cmp.day()), date1);

    let date2 = DateParser::parse("tomorrow");
    let date2cmp = Local::now() + Duration::days(1);

    assert_eq!(NaiveDate::from_ymd_opt(date2cmp.year(), date2cmp.month(), date2cmp.day()), date2);
}

#[test]
fn test_parser()
{
    let mut session = Session {
        todo: Todo {
            summary: "".to_string(),
            due: None
        },
        config: crate::Config { 
            url: "https://foo".to_string(), 
            username: "".to_string(), 
            password: "".to_string(), 
            calendars: Vec::new()
        }
    };

    let ret = parse_task(&"do the thing @tomorrow".to_string(), &mut session);
    assert!(ret.is_ok());

    assert_eq!("do the thing", session.todo.summary);
    let tomorrow = DateParser::parse("tomorrow");
    assert_eq!(tomorrow, session.todo.due);
}

#[test]
fn test_incremental_parser()
{
    let mut session = Session {
        todo: Todo {
            summary: "".to_string(),
            due: None
        },
        config: crate::Config { 
            url: "https://foo".to_string(), 
            username: "".to_string(), 
            password: "".to_string(), 
            calendars: Vec::new()
        }
    };

    let mut task = "".to_string();
    for chr in "do the thing @tomorrow".chars()
    {
        task.push(chr);
        let ret = 
            parse_task(&task, &mut session);
        assert!(ret.is_ok());
    }

    assert_eq!("do the thing", session.todo.summary);
    let tomorrow = DateParser::parse("tomorrow");
    assert_eq!(tomorrow, session.todo.due);
}

#[test]
fn test_gen_caldav()
{
  let todo = Todo { summary: "Hello World".to_string(), due: DateParser::parse("tomorrow") };
  let uuid = Uuid::new_v4();

  let test = gen_caldav(&todo, uuid);

  assert!(test.contains("SUMMARY:Hello World"));

  
}
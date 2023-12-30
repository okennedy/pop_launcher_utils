# Pop Launch Utils

Some simple launcher addons for [pop-launcher](https://docs.rs/crate/pop-launcher/).

This is part of my effort to learn Rust, so don't expect anything particularly fancy.

### pass

**Usage:** `pass [password]`

**Requires:** The [pass](https://www.passwordstore.org/) utility, which in turn requires xclip.

The utility will do a (case sensitive) search for the phrase `password` anywhere in the full path of
a password file.  Selecting a password will invoke `pass -c` on the file.

### todo

**Usage:** `todo [task description] @[date]`

**Requires:** A config file `~/.config/pop_todo.json` (see below)

This utility lets you quickly enter TODO items and send them as Tasks to a CalDAV server.  The 
search phrase entered into the launcher will be parsed into a todo quick entry.  Special prefixes
are used to set fields of the todo.

- `@`: Sets the due date.  Dates can be entered as `mm-dd-yyyy` or using natural language, with `_` instead of space.  For example: `@today`, `@tomorrow`, `@next_week`, `@july_20`

**Examples:**

* `todo create a README file @tomorrow`

**Configuration**

Create a file: `~/.config/pop_todo.json`
```
{
    "url" : "https://your.caldav.server/remote.php/dav",
    "username" : "your_username",
    "password" : "super_secret",
    "calendars" : [
      "cal_id_1",
      "cal_id_2",
      "cal_id_3",
      "..."
    ]
}
```

* `url`: The base CalDAV URL of your server; Tested with Nextcloud.  In Nextcloud, you can get this URL by going to the Calendars page, and choosing settings.
* `username`: Your CalDAV username.  This is your Nextcloud username.
* `password`: Your CalDAV password.  In Nextcloud, you are encouraged to use an 'app-specific-password', which you can create through the Security settings panel.
* `calendars`: A list of calendar-ids to include in the list.  In Nextcloud, this is *usually* the fully lower case name of your calendar; You can get this by going to the calendar page and looking at the URL. 

# Time Tracker

> Track what you work on, automatically - without getting in your way

An activity tracker designed for the workplace, yet flexible enough to use anywhere.

<!-- ## Getting started -->
<!-- ## Install -->
<!-- ## Run -->

## Features
- **Start/Stop Tasks** - choose what you work on and when
- **Automatic Activity Tracking** - records keyboard and mouse activity
- **Minimal Setup** - no extra dependencies required
- **Local Database Storage** - works offline
- **System Tray Support** - keeps your task bar uncluttered

## Future Features
- Custom activity timeout
- Discord Rich Presence integration
- Sync with online business databases

## Database

### The diagram below shows the minimal schema required:

<div style="display: flex; justify-content: center">
    <img src="media/schema.svg" alt="database schema" width=500/>
</div>

### Full SQL schema
[schema.sql](assets/schema.sql)

## Privacy

### What data is collected?

> Keyboard and Mouse events are recorded but immediately discarded. Only activity is tracked; no personal metadata is stored.

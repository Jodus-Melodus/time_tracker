CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- The name of the task
    name TEXT NOT NULL DEFAULT '',
    -- Short description
    description TEXT NOT NULL DEFAULT '',
    -- Priority of task [Low, Medium, High]
    priority INTEGER NOT NULL DEFAULT 0,
    -- 0 = Not currently working, 1 = currently working on the task
    in_progress INTEGER NOT NULL DEFAULT 0
);
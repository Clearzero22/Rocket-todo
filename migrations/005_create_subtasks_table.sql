-- Create subtasks table
CREATE TABLE IF NOT EXISTS subtasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_todo_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed')),
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high')),
    due_date DATETIME,
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_todo_id) REFERENCES todos(id) ON DELETE CASCADE
);

-- Create indexes for faster queries
CREATE INDEX IF NOT EXISTS idx_subtasks_parent_todo_id ON subtasks(parent_todo_id);
CREATE INDEX IF NOT EXISTS idx_subtasks_status ON subtasks(status);
CREATE INDEX IF NOT EXISTS idx_subtasks_due_date ON subtasks(due_date);
CREATE INDEX IF NOT EXISTS idx_subtasks_order ON subtasks(parent_todo_id, order_index);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_subtasks_updated_at
    AFTER UPDATE ON subtasks
    FOR EACH ROW
BEGIN
    UPDATE subtasks SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
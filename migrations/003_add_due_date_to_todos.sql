-- Add due_date column to todos table
ALTER TABLE todos ADD COLUMN due_date DATETIME;

-- Create index on due_date for faster queries
CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date);
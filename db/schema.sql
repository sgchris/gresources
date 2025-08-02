CREATE TABLE resources (
    id INTEGER PRIMARY KEY,
    user_id INTEGER DEFAULT 1,
    path TEXT UNIQUE NOT NULL,
    content TEXT,
    size INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_path ON resources(path);

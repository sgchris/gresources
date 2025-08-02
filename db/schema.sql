CREATE TABLE resources (
    id INTEGER PRIMARY KEY,
    user_id INTEGER DEFAULT 1,

    path TEXT UNIQUE NOT NULL COMMENT 'The URI of the resource',

    content TEXT COMMENT 'The content of the resource if it is text-based, otherwise NULL',

    local_path TEXT COMMENT 'Path to the local file or NULL',
    mime_type TEXT COMMENT 'The MIME type of the resource if it is a file, otherwise NULL',    
    
    size INTEGER COMMENT 'Size of the resource in bytes',

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

CREATE INDEX idx_path ON resources(path);

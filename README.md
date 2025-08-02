# GResources

A high-performance RESTful API for managing hierarchical text resources and folders, built with Rust and SQLite.

## Overview

GResources is a lightweight resource management server designed primarily for testing and development workflows. It provides a simple HTTP API to create, read, update, and delete text-based resources organized in a folder structure.

## Key Features

- **RESTful API** - Standard HTTP methods (POST, GET, PATCH, DELETE)
- **Hierarchical Organization** - Nested folders up to 5 levels deep
- **Fast & Concurrent** - Built with Rust and Actix-web for high performance
- **Thread-Safe** - SQLite with proper concurrency handling
- **Comprehensive Logging** - All write operations are logged
- **Simple Setup** - Embedded SQLite database, no external dependencies

## API Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/path/to/resource` | Create a new resource with text content |
| `GET` | `/path/to/resource` | Retrieve resource content and metadata |
| `GET` | `/path/to/folder` | List all resources in a folder |
| `PATCH` | `/path/to/resource` | Update resource content |
| `PATCH` | `/path/to/folder` | Rename folder |
| `DELETE` | `/path/to/resource` | Delete a resource |
| `DELETE` | `/path/to/folder` | Delete empty folder |

## Response Format

### Resource Responses
- **Body**: Resource content (text)
- **Headers**: Metadata including creation time, update time, folder path, and size

### Folder Responses  
- **Body**: List of resources (one per line with full paths)
- **Headers**: Folder metadata including creation time and path

## Limitations

- Text content only (max 5MB per resource)
- Resource names limited to 100 characters
- Maximum folder nesting depth: 5 levels
- No authentication (single user_id: "1")

## Tech Stack

- **Rust** - Core language
- **Actix-web** - HTTP server framework  
- **SQLite** - Embedded database via rusqlite
- **Thread-safe** - Concurrent request handling

## Use Cases

Perfect for:
- Testing and development environments
- Temporary data storage
- API prototyping
- Educational projects
- Local resource management

## Quick Example

```bash
# Create a resource
curl -X POST http://localhost:8080/docs/readme.txt -d "# My Project"

# Retrieve it
curl http://localhost:8080/docs/readme.txt

# List folder contents  
curl http://localhost:8080/docs

# Update content
curl -X PATCH http://localhost:8080/docs/readme.txt -d "# Updated Project"

# Delete resource
curl -X DELETE http://localhost:8080/docs/readme.txt
```

## Status

🚧 **In Development** - Core functionality being implemented


# API Testing Scripts

This directory contains test scripts to verify the GResources API functionality.

## Prerequisites

1. **Start the GResources server** first:
   ```bash
   cargo run
   ```
   The server should be running on `http://127.0.0.1:8080` (or whatever port you configured in Settings.toml)

2. **Ensure curl is installed** on your system

## Running the Tests

### Option 1: Bash Script (Recommended)
```bash
# Make the script executable (Linux/Mac/WSL)
chmod +x test_api.sh

# Run the test
./test_api.sh
```

### Option 2: PowerShell Script (Windows)

**Option 2a: Using Invoke-WebRequest (Recommended for Windows)**
```powershell
# Run the test using PowerShell's built-in web client
.\test_api.ps1
```

**Option 2b: Using actual curl.exe**
```powershell
# Requires curl.exe to be installed and in PATH
.\test_api_curl.ps1
```

### Option 3: Manual Testing
You can also run individual curl commands manually:

```bash
# Test creating a resource
curl -X POST http://127.0.0.1:8080/test/myfile -d "Hello World"

# Test getting a resource
curl http://127.0.0.1:8080/test/myfile

# Test listing folder
curl http://127.0.0.1:8080/test

# Test updating a resource
curl -X PATCH http://127.0.0.1:8080/test/myfile -d "Updated content"

# Test deleting a resource
curl -X DELETE http://127.0.0.1:8080/test/myfile
```

## What the Test Script Does

The test script performs a comprehensive test of all API operations:

1. ✅ **GET non-existent resource** → Should return 404
2. ✅ **POST create resource** → Should return 200
3. ✅ **POST duplicate resource** → Should return 409 (Conflict)
4. ✅ **GET existing resource** → Should return 200 with correct content
5. ✅ **GET folder listing** → Should return 200 and include the resource
6. ✅ **PATCH update resource** → Should return 200
7. ✅ **GET updated resource** → Should return 200 with new content
8. ✅ **DELETE resource** → Should return 200
9. ✅ **GET deleted resource** → Should return 404
10. ✅ **GET folder after deletion** → Should return 200 without the resource

## Expected Output

The script will show colored output indicating:
- ✅ Green checkmarks for successful tests
- ❌ Red X marks for failed tests
- Response details for debugging

## Troubleshooting

- **Connection refused**: Make sure the GResources server is running
- **Port issues**: Check that the port in the script matches your Settings.toml
- **Permission denied**: On Linux/Mac, make sure the script is executable (`chmod +x test_api.sh`)

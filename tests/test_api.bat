@echo off
setlocal enabledelayedexpansion

REM GResources API Test Script - Windows Batch File
REM This script tests all the CRUD operations of the GResources API using CURL

REM Configuration
set BASE_URL=http://127.0.0.1:8002
set TEST_FOLDER=/test-folder

REM Generate a random resource name
set /a RANDOM_ID=%RANDOM% + 1000
set RESOURCE_NAME=test-resource-!RANDOM_ID!
set RESOURCE_PATH=!TEST_FOLDER!/!RESOURCE_NAME!
set FULL_URL=!BASE_URL!!RESOURCE_PATH!
set FOLDER_URL=!BASE_URL!!TEST_FOLDER!

echo === GResources API Test Script ===
echo Testing resource: !RESOURCE_PATH!
echo Server URL: !BASE_URL!
echo.

REM Test content
set ORIGINAL_CONTENT=This is the original content for resource !RESOURCE_NAME!
set UPDATED_CONTENT=This is the UPDATED content for resource !RESOURCE_NAME!

REM Colors for output (Windows compatible)
set GREEN=[32m
set RED=[31m
set YELLOW=[33m
set CYAN=[36m
set GRAY=[37m
set NC=[0m

echo %CYAN%1. Testing GET non-existent resource (should be 404)...%NC%
curl -s -o nul -w "Status: %%{http_code}" -X GET "!FULL_URL!"
echo.
echo.

echo %CYAN%2. Testing POST create resource (should be 201)...%NC%
curl -s -w "Status: %%{http_code}" -X POST "!FULL_URL!" -d "!ORIGINAL_CONTENT!"
echo.
echo.

echo %CYAN%3. Testing POST create same resource again (should be 409 Conflict)...%NC%
curl -s -w "Status: %%{http_code}" -X POST "!FULL_URL!" -d "!ORIGINAL_CONTENT!"
echo.
echo.

echo %CYAN%4. Testing GET existing resource (should be 200)...%NC%
echo Expected content: !ORIGINAL_CONTENT!
echo Actual content:
curl -s -w "Status: %%{http_code}" -X GET "!FULL_URL!"
echo.
echo.

echo %CYAN%5. Testing GET folder listing (should be 200 and contain resource)...%NC%
echo Looking for resource: !RESOURCE_PATH!
echo Folder contents:
curl -s -w "Status: %%{http_code}" -X GET "!FOLDER_URL!"
echo.
echo.

echo %CYAN%6. Testing PATCH update resource (should be 204)...%NC%
curl -s -w "Status: %%{http_code}" -X PATCH "!FULL_URL!" -d "!UPDATED_CONTENT!"
echo.
echo.

echo %CYAN%7. Testing GET updated resource (should be 200 with new content)...%NC%
echo Expected content: !UPDATED_CONTENT!
echo Actual content:
curl -s -w "Status: %%{http_code}" -X GET "!FULL_URL!"
echo.
echo.

echo %CYAN%8. Testing DELETE resource (should be 204)...%NC%
curl -s -w "Status: %%{http_code}" -X DELETE "!FULL_URL!"
echo.
echo.

echo %CYAN%9. Testing GET deleted resource (should be 404)...%NC%
curl -s -o nul -w "Status: %%{http_code}" -X GET "!FULL_URL!"
echo.
echo.

echo %CYAN%10. Testing GET folder listing after deletion (resource should not be listed)...%NC%
echo Resource !RESOURCE_PATH! should NOT be in the list:
curl -s -w "Status: %%{http_code}" -X GET "!FOLDER_URL!"
echo.
echo.

echo %GREEN%=== Test Complete ===%NC%
echo %YELLOW%All tests have been executed. Check the status codes and responses above.%NC%
echo.
echo Expected status codes:
echo - Step 1: 404 (resource doesn't exist)
echo - Step 2: 201 (resource created)
echo - Step 3: 409 (conflict - resource already exists)
echo - Step 4: 200 (resource retrieved)
echo - Step 5: 200 (folder listing)
echo - Step 6: 204 (resource updated)
echo - Step 7: 200 (updated resource retrieved)
echo - Step 8: 204 (resource deleted)
echo - Step 9: 404 (deleted resource not found)
echo - Step 10: 200 (folder listing after deletion)

pause

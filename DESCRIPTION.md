# GResources project description

A resources manager - RESTFul API for CRUD'ing folders and resources.

Built with Rust and SQLite.

Basically it's on online DB, but its main purpose is for tests, at least now.

## Acceptable requests 
- POST - create new resource.
- PATCH - update a resource.
- GET - get resource or list resources if it's a folder
- DELETE - delete a resource. delete an empty folder

## Responses

- For POST and PATCH requests the response is either 200 ok, or 409 conflict (when trying to create existing resource again), or 400 bad request when trying to patch a folder.
- For GET requests, the response is either a list or a specific resource.
    * specific resource - the response body is the content of the resource, and the metadata is defined in the response headers. 
    For example `gresource-created-at: 2025-07-25T18:05:12.45Z`. The timestamps are in ISO 8601 format. The headers
        - `gresource-created-at: 2025-07-25T18:05:12.45Z`
        - `gresource-updated-at: 2025-07-25T19:15:02.41Z`
        - `gresource-folder: /myresources/sub-type`
        - `gresource-size: 34250` 
    * specific folder - the response body contains list of resources under that folder - each resource on a separate line. resources' full paths. The metadata contains the created-at property and the folder.
        - headers example:
            - `gresource-created-at: 2025-07-25T18:05:12.45Z`
            - `gresource-folder: /myresources`
        - body example:<br>
            /myresources/resource1<br>
            /myresources/resource2<br>
            /myresources/sub-type/res1<br>


## Example of requests and the expected results

### Single resource
- sending POST to /myresource with request body "my request body", will generate a resource on the server with the content "my request body" and respond with 200 ok
- sending GET to /myresource will respond with 200 ok, the metadata of the resource and "my request body" as a response body. The metadata includes created at, updated at, folder, size in bytes.
- sending POST to /myresource again will produce 409 conflict response, since the resource already exists. Updating resources is allowed only using PATCH requests.
- sending PATCH to /myresource with body "my updated body" will update the resource content
- sending GET to /myresource again after the patch, will respond with 200 and "my updated body" as a response body.
- sending GET to /non-existing-resource will respond with 404 not found.

### Folders
So far I showed exaples for a single resource. There's an option to create folders

- sending POST to /myresources/resource1 and /myresources/resource2 creates resource1 and resource2 under myresources folder
- sending GET to /myresources returns a list of resources under that folder

Folders may be nested
- sending POST to /myresources/sub-type/res1 creates a resource res1 under sub-type that is under myresources.
- sending GET to /myresources returns a list with resource1, resource2, sub-type folder and sub-type/res1 resource.

Delete is allowed on resources and empty folders only
- sending DELETE to /myresources/sub-type returns 400 bad request since the folder isn't empty

The correct way:
- sending DELETE to /myresources/sub-type/res1 returns 200 ok
- sending DELETE to /myresources/sub-type returns now 200 ok, since there are no resources under it

Patch requests
- Patch requests may be sent to a specific resource to update its content
- Patch requests may be sent to a specific folder to update its name

## The app

### The tech stack

- Rust
- SQLite, using `rusqlite` crate
- Actix (`actix-web` crate)  for the web server

### The design/flow

All the resources are managed through the DB. The [schema file](db/schema.sql).

The service must be super fast and efficient yet robust and stable. It must support many requests at a time.

Make sure SQLite is thread safe in this app.

Avoid conflicts - when more than one async flow tries to access the DB, one accesses and the others wait until the db is released.

The app should log every write operation (POST, PATCH, DELETE). Use the most common logs crate/mechanism. Make sure there are no conflicts and several async flows can write logs simultaneously. The log file should be in the relevant folder (e.g. in windows it's under AppData/Local)

That means that for every incoming request, the followint steps occur:
- Validate the request 
    - basic validation
    - ensure that the "status" of the resource is valid for the request (e.g. that the resource doesn't exist for POST requests, or exists for PATCH requests, or exists for GET requests (otherwise 404), etc.)
- Update the DB
    - create a new resource, or delete it, or update resource contents - the timestamps, the size, etc.
- respond accordingly

## Limitation

- The content of a resource is textual only (currently)
- max resource name is at most 100 chars
- max resource size is 5MB
- max nested folders 5. i.e. at most `/fold1/subfold1/subfold2/subfold3/subfold4`
- no authentication/authorization at this time. The support will be added later. But, for future support, all the resources have 'user_id' field with the value of "1".

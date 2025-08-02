# GResources project description

A resources manager - RESTFul API for CRUD'ing folders and resources.

Built with Rust and SQLite.

Main purpose is for tests, rather than using it as an online DB.

## acceptable requests 
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

### single resource
- sending POST to /myresource with request body "my request body", will generate a resource on the server with the content "my request body" and respond with 200 ok
- sending GET to /myresource will respond with 200 ok, the metadata of the resource and "my request body" as a response body. The metadata includes created at, updated at, folder, size in bytes.
- sending POST to /myresource again will produce 409 conflict response, since the resource already exists. Updating resources is allowed only using PATCH requests.
- sending PATCH to /myresource with body "my updated body" will update the resource content
- sending GET to /myresource again after the patch, will respond with 200 and "my updated body" as a response body.
- sending GET to /non-existing-resource will respond with 404 not found.

### folders
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

### The tech

- Rust
- SQLite, using `rusqlite` crate
- Actix (`actix-web` crate)  for the web server

### The design

All the resources are managed through the DB. Please take a look at the [schema file](db/schema.sql).
That means that for every incoming request, the steps:
- Validate the request 
    - basic validation
    - ensure that the resource' status reflects the request (doesn't exist for POST to resource, or exists upon PATCH requests, exists for GET requests, etc.)
- Update the DB
    - update resource contents, the timestamps, the size, etc.


## Limitation

- The content of a resource is textual only
- max resource name is 100 chars
- max resource size is 5MB
- max nested folders 5. i.e. at most `/fold1/subfold1/subfold2/.../subfold4`
- no authentication/authorization for the first version. The support will be added later on. all the resources have 'user_id' field with the value of "1".

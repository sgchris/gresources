use crate::database::Database;
use crate::logging::Logger;
use crate::models::Resource;
use crate::validation::{normalize_path, validate_content, validate_path};
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<Database>,
    pub logger: Arc<Logger>,
}

pub async fn handle_post(
    req: HttpRequest,
    body: String,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    data.logger.log_info(&"");
    data.logger.log_info(&format!("POST request received for path: {}", path));
    data.logger.log_debug(&format!("Request body length: {} bytes", body.len()));

    // Validate path and content
    data.logger.log_debug(&format!("Validating path: {}", path));
    if let Err(e) = validate_path(&path) {
        data.logger.log_warn(&format!("Path validation failed for {}: {}", path, e));
        data.logger.log_write_operation("POST", &path, false);
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }
    data.logger.log_debug("Path validation successful");

    data.logger.log_debug(&format!("Validating content (length: {})", body.len()));
    if let Err(e) = validate_content(&body) {
        data.logger.log_warn(&format!("Content validation failed for {}: {}", path, e));
        data.logger.log_write_operation("POST", &path, false);
        return Ok(HttpResponse::BadRequest().body(format!("Invalid content: {}", e)));
    }
    data.logger.log_debug("Content validation successful");

    // Check if resource already exists
    data.logger.log_debug(&format!("Checking if resource exists: {}", path));
    match data.db.resource_exists(&path) {
        Ok(true) => {
            data.logger.log_info(&format!("Resource already exists, returning conflict: {}", path));
            data.logger.log_write_operation("POST", &path, false);
            return Ok(HttpResponse::Conflict().body("Resource already exists"));
        }
        Ok(false) => {
            data.logger.log_debug("Resource does not exist, proceeding with creation");
        }
        Err(e) => {
            data.logger.log_error(&format!("Database error while checking resource existence for {}: {}", path, e));
            data.logger.log_write_operation("POST", &path, false);
            return Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)));
        }
    }

    // Create the resource
    data.logger.log_debug(&format!("Creating new resource: {}", path));
    let resource = Resource::new(path.clone(), body);

    match data.db.create_resource(&resource) {
        Ok(_) => {
            data.logger.log_info(&format!("Resource created successfully: {}", path));
            data.logger.log_write_operation("POST", &path, true);
            Ok(HttpResponse::Created().finish()) // 201 Created with no body
        }
        Err(e) => {
            data.logger.log_error(&format!("Failed to create resource {}: {}", path, e));
            data.logger.log_write_operation("POST", &path, false);
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to create resource: {}", e)))
        }
    }
}

pub async fn handle_get(req: HttpRequest, data: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    data.logger.log_info(&"");
    data.logger.log_info(&format!("GET request received for path: {}", path));

    data.logger.log_debug(&format!("Validating path: {}", path));
    if let Err(e) = validate_path(&path) {
        data.logger.log_warn(&format!("Path validation failed for {}: {}", path, e));
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }
    data.logger.log_debug("Path validation successful");

    // Try to get resource first
    data.logger.log_debug(&format!("Attempting to get resource: {}", path));
    match data.db.get_resource(&path) {
        Ok(Some(resource)) => {
            data.logger.log_info(&format!("Resource found: {}, size: {} bytes", path, resource.size));
            let mut response = HttpResponse::Ok();

            // Add metadata headers
            response.insert_header((
                "gresource-created-at",
                resource
                    .created_at
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string(),
            ));
            response.insert_header((
                "gresource-updated-at",
                resource
                    .updated_at
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string(),
            ));
            response.insert_header(("gresource-folder", resource.get_folder_path()));
            response.insert_header(("gresource-size", resource.size.to_string()));

            data.logger.log_debug(&format!("Returning resource content for: {}", path));
            Ok(response.body(resource.content.unwrap_or_default()))
        }
        Ok(None) => {
            data.logger.log_debug(&format!("Resource not found, attempting to list as folder: {}", path));
            // Try to list as folder
            match data.db.list_folder_resources(&path) {
                Ok(folder_info) => {
                    data.logger.log_info(&format!("Folder found: {}, contains {} resources", path, folder_info.resources.len()));
                    let mut response = HttpResponse::Ok();

                    // Add folder metadata headers
                    response.insert_header((
                        "gresource-created-at",
                        folder_info
                            .created_at
                            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                            .to_string(),
                    ));
                    response.insert_header(("gresource-folder", folder_info.path));

                    // Create response body with resource paths
                    let body = folder_info.resources.join("\n");
                    data.logger.log_debug(&format!("Returning folder listing for: {} with {} resources", path, folder_info.resources.len()));
                    Ok(response.body(body))
                }
                Err(e) => {
                    data.logger.log_info(&format!("Neither resource nor folder found: {}, error: {}", path, e));
                    Ok(HttpResponse::NotFound().body("Resource not found"))
                }
            }
        }
        Err(e) => {
            data.logger.log_error(&format!("Database error while getting resource {}: {}", path, e));
            Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)))
        }
    }
}

pub async fn handle_patch(
    req: HttpRequest,
    body: String,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    data.logger.log_info(&"");
    data.logger.log_info(&format!("PATCH request received for path: {}", path));
    data.logger.log_debug(&format!("Request body length: {} bytes", body.len()));

    data.logger.log_debug(&format!("Validating path: {}", path));
    if let Err(e) = validate_path(&path) {
        data.logger.log_warn(&format!("Path validation failed for {}: {}", path, e));
        data.logger.log_write_operation("PATCH", &path, false);
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }
    data.logger.log_debug("Path validation successful");

    data.logger.log_debug(&format!("Validating content (length: {})", body.len()));
    if let Err(e) = validate_content(&body) {
        data.logger.log_warn(&format!("Content validation failed for {}: {}", path, e));
        data.logger.log_write_operation("PATCH", &path, false);
        return Ok(HttpResponse::BadRequest().body(format!("Invalid content: {}", e)));
    }
    data.logger.log_debug("Content validation successful");

    // Check if resource exists
    data.logger.log_debug(&format!("Checking if resource exists: {}", path));
    match data.db.resource_exists(&path) {
        Ok(false) => {
            data.logger.log_info(&format!("Resource not found for PATCH: {}", path));
            data.logger.log_write_operation("PATCH", &path, false);
            return Ok(HttpResponse::NotFound().body("Resource not found"));
        }
        Ok(true) => {
            data.logger.log_debug("Resource exists, proceeding with update");
        }
        Err(e) => {
            data.logger.log_error(&format!("Database error while checking resource existence for {}: {}", path, e));
            data.logger.log_write_operation("PATCH", &path, false);
            return Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)));
        }
    }

    // Update the resource
    data.logger.log_debug(&format!("Updating resource: {}", path));
    match data.db.update_resource(&path, &body) {
        Ok(_) => {
            data.logger.log_info(&format!("Resource updated successfully: {}", path));
            data.logger.log_write_operation("PATCH", &path, true);
            Ok(HttpResponse::NoContent().finish()) // 204 No Content with no body
        }
        Err(e) => {
            data.logger.log_error(&format!("Failed to update resource {}: {}", path, e));
            data.logger.log_write_operation("PATCH", &path, false);
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to update resource: {}", e)))
        }
    }
}

pub async fn handle_delete(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    data.logger.log_info(&"");
    data.logger.log_info(&format!("DELETE request received for path: {}", path));

    data.logger.log_debug(&format!("Validating path: {}", path));
    if let Err(e) = validate_path(&path) {
        data.logger.log_warn(&format!("Path validation failed for {}: {}", path, e));
        data.logger.log_write_operation("DELETE", &path, true);
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }
    data.logger.log_debug("Path validation successful");

    // Check if it's a resource
    data.logger.log_debug(&format!("Checking if path is a resource: {}", path));
    match data.db.get_resource(&path) {
        Ok(Some(_)) => {
            data.logger.log_info(&format!("Found resource to delete: {}", path));
            // It's a resource, delete it
            match data.db.delete_resource(&path) {
                Ok(_) => {
                    data.logger.log_info(&format!("Resource deleted successfully: {}", path));
                    data.logger.log_write_operation("DELETE", &path, true);
                    Ok(HttpResponse::Ok().finish()) // 200 OK with no body
                }
                Err(e) => {
                    data.logger.log_error(&format!("Failed to delete resource {}: {}", path, e));
                    data.logger.log_write_operation("DELETE", &path, false);
                    Ok(HttpResponse::InternalServerError()
                        .body(format!("Failed to delete resource: {}", e)))
                }
            }
        }
        Ok(None) => {
            data.logger.log_debug(&format!("Path is not a resource, checking if it's an empty folder: {}", path));
            // Check if it's an empty folder
            match data.db.folder_is_empty(&path) {
                Ok(true) => {
                    data.logger.log_info(&format!("Found empty folder to delete: {}", path));
                    // It's an empty folder, we can "delete" it (no actual deletion needed since folders are implicit)
                    data.logger.log_write_operation("DELETE", &path, true);
                    Ok(HttpResponse::Ok().finish()) // 200 OK with no body
                }
                Ok(false) => {
                    data.logger.log_warn(&format!("Attempt to delete non-empty folder: {}", path));
                    Ok(HttpResponse::BadRequest().body("Cannot delete non-empty folder"))
                }
                Err(e) => {
                    data.logger.log_info(&format!("Path not found as resource or folder: {}, error: {}", path, e));
                    Ok(HttpResponse::NotFound().body("Resource or folder not found"))
                }
            }
        }
        Err(e) => {
            data.logger.log_error(&format!("Database error while checking resource {}: {}", path, e));
            data.logger.log_write_operation("DELETE", &path, false);
            Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)))
        }
    }
}

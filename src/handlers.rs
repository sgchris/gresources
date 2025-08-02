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

    // Log the operation start
    data.logger.log_write_operation("POST", &path, false);

    // Validate path and content
    if let Err(e) = validate_path(&path) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }

    if let Err(e) = validate_content(&body) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid content: {}", e)));
    }

    // Check if resource already exists
    match data.db.resource_exists(&path) {
        Ok(true) => {
            return Ok(HttpResponse::Conflict().body("Resource already exists"));
        }
        Ok(false) => {}
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)));
        }
    }

    // Create the resource
    let resource = Resource::new(path.clone(), body);

    match data.db.create_resource(&resource) {
        Ok(_) => {
            data.logger.log_write_operation("POST", &path, true);
            Ok(HttpResponse::Created().finish()) // 201 Created with no body
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to create resource: {}", e)))
        }
    }
}

pub async fn handle_get(req: HttpRequest, data: web::Data<AppState>) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    if let Err(e) = validate_path(&path) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }

    // Try to get resource first
    match data.db.get_resource(&path) {
        Ok(Some(resource)) => {
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

            Ok(response.body(resource.content.unwrap_or_default()))
        }
        Ok(None) => {
            // Try to list as folder
            match data.db.list_folder_resources(&path) {
                Ok(folder_info) => {
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
                    Ok(response.body(body))
                }
                Err(_) => Ok(HttpResponse::NotFound().body("Resource not found")),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e))),
    }
}

pub async fn handle_patch(
    req: HttpRequest,
    body: String,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let path = normalize_path(req.path());

    // Log the operation start
    data.logger.log_write_operation("PATCH", &path, false);

    if let Err(e) = validate_path(&path) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }

    if let Err(e) = validate_content(&body) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid content: {}", e)));
    }

    // Check if resource exists
    match data.db.resource_exists(&path) {
        Ok(false) => {
            return Ok(HttpResponse::NotFound().body("Resource not found"));
        }
        Ok(true) => {}
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e)));
        }
    }

    // Update the resource
    match data.db.update_resource(&path, &body) {
        Ok(_) => {
            data.logger.log_write_operation("PATCH", &path, true);
            Ok(HttpResponse::NoContent().finish()) // 204 No Content with no body
        }
        Err(e) => {
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

    // Log the operation start
    data.logger.log_write_operation("DELETE", &path, false);

    if let Err(e) = validate_path(&path) {
        return Ok(HttpResponse::BadRequest().body(format!("Invalid path: {}", e)));
    }

    // Check if it's a resource
    match data.db.get_resource(&path) {
        Ok(Some(_)) => {
            // It's a resource, delete it
            match data.db.delete_resource(&path) {
                Ok(_) => {
                    data.logger.log_write_operation("DELETE", &path, true);
                    Ok(HttpResponse::NoContent().finish()) // 204 No Content with no body
                }
                Err(e) => Ok(HttpResponse::InternalServerError()
                    .body(format!("Failed to delete resource: {}", e))),
            }
        }
        Ok(None) => {
            // Check if it's an empty folder
            match data.db.folder_is_empty(&path) {
                Ok(true) => {
                    // It's an empty folder, we can "delete" it (no actual deletion needed since folders are implicit)
                    data.logger.log_write_operation("DELETE", &path, true);
                    Ok(HttpResponse::NoContent().finish()) // 204 No Content with no body
                }
                Ok(false) => Ok(HttpResponse::BadRequest().body("Cannot delete non-empty folder")),
                Err(_) => Ok(HttpResponse::NotFound().body("Resource or folder not found")),
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Database error: {}", e))),
    }
}

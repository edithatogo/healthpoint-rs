//! Validation helpers for externally supplied query and resource parameters.

use crate::{
    error::{HealthpointError, Result},
    query::{GeoPoint, ServiceQuery},
};

/// Validate a FHIR resource id before it is appended as a URL path segment.
///
/// FHIR ids are case-sensitive strings containing ASCII letters, digits, hyphen, and dot, with a
/// maximum length of 64 characters. Keeping this validation near the domain model avoids path
/// traversal and accidental URL-shape changes in client adapters.
pub fn validate_resource_id(id: &str) -> Result<()> {
    let valid_len = (1..=64).contains(&id.len());
    let valid_chars = id
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.'));
    let not_dot_segment = id != "." && id != "..";
    if valid_len && valid_chars && not_dot_segment {
        Ok(())
    } else {
        Err(HealthpointError::InvalidInput(format!(
            "invalid FHIR resource id {id:?}; expected 1..64 ASCII letters/digits/dot/hyphen"
        )))
    }
}

/// Validate latitude/longitude.
pub fn validate_geo_point(point: GeoPoint) -> Result<()> {
    if !(-90.0..=90.0).contains(&point.lat) || !(-180.0..=180.0).contains(&point.lon) {
        return Err(HealthpointError::InvalidInput(format!(
            "invalid latitude/longitude {}, {}; expected lat -90..90 and lon -180..180",
            point.lat, point.lon
        )));
    }
    Ok(())
}

/// Validate radius in kilometres.
pub fn validate_radius_km(radius: f32) -> Result<()> {
    if radius.is_finite() && radius > 0.0 && radius <= 500.0 {
        Ok(())
    } else {
        Err(HealthpointError::InvalidInput(format!(
            "invalid radius {radius}; expected > 0 and <= 500 km"
        )))
    }
}

/// Validate a service query before sending it to an HTTP adapter.
pub fn validate_service_query(query: &ServiceQuery) -> Result<()> {
    if let Some(point) = query.nearby {
        validate_geo_point(point)?;
    }
    if let Some(radius) = query.radius_km {
        validate_radius_km(radius)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_fhir_resource_ids() {
        assert!(validate_resource_id("abc-123.def").is_ok());
        assert!(validate_resource_id("Location/abc").is_err());
        assert!(validate_resource_id("..").is_err());
        assert!(validate_resource_id("").is_err());
    }

    #[test]
    fn validates_geo_ranges() {
        assert!(
            validate_geo_point(GeoPoint {
                lat: -36.8,
                lon: 174.7
            })
            .is_ok()
        );
        assert!(
            validate_geo_point(GeoPoint {
                lat: -100.0,
                lon: 174.7
            })
            .is_err()
        );
    }
}

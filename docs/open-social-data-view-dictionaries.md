# open_social_data view dictionaries

These dictionaries describe the current string-row views emitted by `healthpoint-osd-adapter`. They are integration contracts, not statements that Healthpoint-derived data is open.

## services

| Column | Description |
| --- | --- |
| `id` | FHIR HealthcareService id. |
| `name` | Service name. |
| `active` | Source active flag. |
| `provided_by_reference` | FHIR Organization reference. |
| `appointment_required` | Source appointment requirement flag. |
| `comment` | Source service comment. |
| `retrieved_at` | Retrieval/projection timestamp from provenance. |

## locations

| Column | Description |
| --- | --- |
| `id` | FHIR Location id. |
| `name` | Location name. |
| `status` | FHIR location status. |
| `mode` | FHIR location mode. |
| `address_text` | Human-readable address. |
| `latitude` | Location latitude, if supplied. |
| `longitude` | Location longitude, if supplied. |
| `managing_organization_reference` | FHIR Organization reference. |
| `retrieved_at` | Retrieval/projection timestamp. |

## organizations

| Column | Description |
| --- | --- |
| `id` | FHIR Organization id. |
| `name` | Organization name. |
| `active` | Source active flag. |
| `aliases` | Semicolon-separated aliases. |
| `part_of_reference` | Parent Organization reference. |
| `retrieved_at` | Retrieval/projection timestamp. |

## service-locations

| Column | Description |
| --- | --- |
| `service_id` | FHIR HealthcareService id. |
| `location_reference` | FHIR Location reference. |
| `location_display` | Source display text. |

## service-codes

| Column | Description |
| --- | --- |
| `service_id` | FHIR HealthcareService id. |
| `field` | Source field: category, type, specialty, service_provision, program, characteristic, communication, referral_method. |
| `system` | Coding system URI. |
| `code` | Code value. |
| `display` | Coding display text. |

## service-contacts

| Column | Description |
| --- | --- |
| `service_id` | FHIR HealthcareService id. |
| `system` | Contact system: phone, email, url, etc. |
| `value` | Contact value. |
| `use` | Contact use. |

## service-eligibilities

| Column | Description |
| --- | --- |
| `service_id` | FHIR HealthcareService id. |
| `eligibility_index` | Zero-based eligibility component index. |
| `system` | Eligibility coding system. |
| `code` | Eligibility code. |
| `display` | Eligibility display. |
| `comment` | Eligibility comment/markdown. |

## service-availability

| Column | Description |
| --- | --- |
| `service_id` | FHIR HealthcareService id. |
| `availability_index` | Zero-based availability index. |
| `days_of_week` | Semicolon-separated FHIR daysOfWeek values. |
| `all_day` | Source all-day flag. |
| `available_start_time` | Opening/start time. |
| `available_end_time` | Closing/end time. |

//! Maps OCSF attribute types to Protocol Buffer type strings.
//!
//! # Type Mapping Table
//!
//! | OCSF type | Proto type | Notes |
//! |-----------|-----------|-------|
//! | `string_t`, `hostname_t`, `ip_t`, `mac_t`, `url_t`, `email_t`, `uuid_t`, etc. | `string` | All string-like types |
//! | `integer_t`, `port_t` | `int32` | |
//! | `long_t`, `timestamp_t` | `int64` | |
//! | `float_t` | `double` | |
//! | `boolean_t` | `bool` | |
//! | `json_t` | `string` | NOT `google.protobuf.Struct` (prost serde incompatible) |
//! | `object_t` | message reference | Handled separately by the codegen module |
//! | Unknown types | `string` | Fallback with warning |

/// Map an OCSF type name to a proto3 scalar type string.
///
/// Returns `None` for `object_t` — object references must be resolved
/// by the caller using the attribute's `object_type` field.
///
/// Returns `"string"` as a fallback for unrecognized types.
pub fn ocsf_to_proto_type(type_name: &str) -> Option<&'static str> {
    let proto = match type_name {
        // String family — all string-like OCSF types.
        "string_t" | "hostname_t" | "ip_t" | "mac_t" | "url_t" | "email_t" | "file_path_t"
        | "file_name_t" | "path_t" | "subnet_t" | "uuid_t" | "username_t" | "process_name_t"
        | "resource_uid_t" | "datetime_t" => "string",

        // json_t maps to string, NOT google.protobuf.Struct.
        // prost_types::Struct does not implement serde traits, breaking
        // #[derive(Serialize, Deserialize)] on generated Rust types.
        "json_t" => "string",

        // Integer family.
        "integer_t" | "port_t" => "int32",

        // Long / timestamp family.
        "long_t" | "timestamp_t" => "int64",

        // Float.
        "float_t" => "double",

        // Boolean.
        "boolean_t" => "bool",

        // Object references — the caller must handle these.
        "object_t" => return None,

        // Fallback: unknown types emit as string.
        _ => "string",
    };
    Some(proto)
}

/// Convert a snake_case OCSF name to PascalCase for proto message names.
///
/// Handles extension-prefixed names by stripping the prefix:
/// - `"network_endpoint"` → `"NetworkEndpoint"`
/// - `"win/win_service"` → `"WinService"` (prefix stripped)
pub fn to_pascal_case(s: &str) -> String {
    // Strip extension prefix (e.g., "win/win_service" → "win_service").
    let name = s.rsplit('/').next().unwrap_or(s);
    name.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert a name to SCREAMING_SNAKE_CASE for proto enum type names.
///
/// - `"authentication"` → `"AUTHENTICATION"`
/// - `"security_finding"` → `"SECURITY_FINDING"`
pub fn to_screaming_snake(s: &str) -> String {
    s.to_uppercase()
}

/// Sanitize an OCSF object name for use as a lookup key.
///
/// Strips extension prefixes: `"win/win_service"` → `"win_service"`.
pub fn sanitize_object_name(s: &str) -> String {
    s.rsplit('/').next().unwrap_or(s).to_string()
}

/// Convert a human-readable caption to a SCREAMING_SNAKE enum variant name.
///
/// - `"Logon"` → `"LOGON"`
/// - `"Service Ticket Request"` → `"SERVICE_TICKET_REQUEST"`
/// - `"TLP:AMBER+STRICT"` → `"TLP_AMBER_STRICT"`
///
/// Non-alphanumeric characters are replaced with `_`, consecutive
/// underscores are collapsed, and leading/trailing underscores are trimmed.
pub fn to_enum_variant_name(caption: &str) -> String {
    caption
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_uppercase().next().unwrap_or(c)
            } else {
                '_'
            }
        })
        .collect::<String>()
        .replace("__", "_")
        .trim_matches('_')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_type_mapping() {
        assert_eq!(ocsf_to_proto_type("string_t"), Some("string"));
        assert_eq!(ocsf_to_proto_type("integer_t"), Some("int32"));
        assert_eq!(ocsf_to_proto_type("long_t"), Some("int64"));
        assert_eq!(ocsf_to_proto_type("timestamp_t"), Some("int64"));
        assert_eq!(ocsf_to_proto_type("float_t"), Some("double"));
        assert_eq!(ocsf_to_proto_type("boolean_t"), Some("bool"));
        assert_eq!(ocsf_to_proto_type("port_t"), Some("int32"));
    }

    #[test]
    fn string_family_mapping() {
        for t in &[
            "hostname_t",
            "ip_t",
            "mac_t",
            "url_t",
            "email_t",
            "uuid_t",
            "file_path_t",
            "subnet_t",
        ] {
            assert_eq!(
                ocsf_to_proto_type(t),
                Some("string"),
                "expected string for {t}"
            );
        }
    }

    #[test]
    fn json_t_maps_to_string() {
        assert_eq!(ocsf_to_proto_type("json_t"), Some("string"));
    }

    #[test]
    fn object_t_returns_none() {
        assert_eq!(ocsf_to_proto_type("object_t"), None);
    }

    #[test]
    fn unknown_type_falls_back_to_string() {
        assert_eq!(ocsf_to_proto_type("some_future_type"), Some("string"));
    }

    #[test]
    fn pascal_case_conversion() {
        assert_eq!(to_pascal_case("network_endpoint"), "NetworkEndpoint");
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("auth_factor"), "AuthFactor");
        assert_eq!(to_pascal_case("cis_csc"), "CisCsc");
    }

    #[test]
    fn pascal_case_strips_extension_prefix() {
        assert_eq!(to_pascal_case("win/win_service"), "WinService");
        assert_eq!(to_pascal_case("win/reg_key"), "RegKey");
    }

    #[test]
    fn screaming_snake_conversion() {
        assert_eq!(to_screaming_snake("authentication"), "AUTHENTICATION");
        assert_eq!(to_screaming_snake("security_finding"), "SECURITY_FINDING");
    }

    #[test]
    fn enum_variant_name_conversion() {
        assert_eq!(to_enum_variant_name("Logon"), "LOGON");
        assert_eq!(
            to_enum_variant_name("Service Ticket Request"),
            "SERVICE_TICKET_REQUEST"
        );
        assert_eq!(to_enum_variant_name("TLP:AMBER+STRICT"), "TLP_AMBER_STRICT");
        assert_eq!(to_enum_variant_name("Unknown"), "UNKNOWN");
        assert_eq!(to_enum_variant_name("Other"), "OTHER");
    }

    #[test]
    fn sanitize_object_name_strips_prefix() {
        assert_eq!(sanitize_object_name("win/win_service"), "win_service");
        assert_eq!(sanitize_object_name("user"), "user");
    }
}

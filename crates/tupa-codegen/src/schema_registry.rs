use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub version: String,
    pub schema: Schema,
    pub migrations: Vec<Migration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: HashMap<String, FieldDef>,
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub type_name: String,
    pub optional: bool,
    pub deprecated: bool,
    pub version_added: Option<String>,
    pub version_deprecated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub from_version: String,
    pub to_version: String,
    pub transforms: Vec<Transform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub op: TransformOp,
    pub field: String,
    pub args: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformOp {
    Rename,
    AddDefault,
    Remove,
    Convert,
}

pub struct SchemaRegistry {
    schemas: HashMap<String, SchemaVersion>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, version: SchemaVersion) {
        self.schemas.insert(name, version);
    }

    pub fn get(&self, name: &str) -> Option<&SchemaVersion> {
        self.schemas.get(name)
    }

    pub fn list_versions(&self, _name: &str) -> Vec<String> {
        vec!["registered".to_string()]
    }

    pub fn allow_runtime_field_insertion(
        &self,
        schema: &Schema,
        field: &str,
    ) -> Result<(), String> {
        if schema.fields.contains_key(field) {
            return Err(format!("Field '{}' already exists", field));
        }
        Ok(())
    }

    pub fn emit_deprecation_warning(&self, field: &str, version: &str) {
        eprintln!(
            "Warning: Field '{}' deprecated in version {}",
            field, version
        );
    }

    pub fn diff_schemas(&self, old: &Schema, new: &Schema) -> SchemaDiff {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();

        for (name, field) in &new.fields {
            if !old.fields.contains_key(name) {
                added.push(name.clone());
            } else if let Some(old_field) = old.fields.get(name) {
                if old_field.type_name != field.type_name {
                    changed.push(name.clone());
                }
            }
        }

        for name in old.fields.keys() {
            if !new.fields.contains_key(name) {
                removed.push(name.clone());
            }
        }

        SchemaDiff {
            added,
            removed,
            changed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchemaDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
}

impl SchemaDiff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_registry_new() {
        let registry = SchemaRegistry::new();
        assert!(registry.schemas.is_empty());
    }

    #[test]
    fn test_schema_registry_register_and_get() {
        let mut registry = SchemaRegistry::new();

        let schema = Schema {
            fields: HashMap::new(),
            nullable: false,
        };

        let version = SchemaVersion {
            version: "1.0".to_string(),
            schema,
            migrations: vec![],
        };

        registry.register("test_pipeline".to_string(), version);

        let retrieved = registry.get("test_pipeline");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().version, "1.0");
    }

    #[test]
    fn test_schema_diff_empty() {
        let registry = SchemaRegistry::new();

        let schema1 = Schema {
            fields: HashMap::new(),
            nullable: false,
        };

        let schema2 = Schema {
            fields: HashMap::new(),
            nullable: false,
        };

        let diff = registry.diff_schemas(&schema1, &schema2);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_schema_diff_detects_added() {
        let registry = SchemaRegistry::new();

        let mut fields1 = HashMap::new();
        fields1.insert(
            "existing".to_string(),
            FieldDef {
                type_name: "string".to_string(),
                optional: true,
                deprecated: false,
                version_added: None,
                version_deprecated: None,
            },
        );

        let mut fields2 = fields1.clone();
        fields2.insert(
            "new_field".to_string(),
            FieldDef {
                type_name: "i64".to_string(),
                optional: false,
                deprecated: false,
                version_added: Some("1.1".to_string()),
                version_deprecated: None,
            },
        );

        let schema1 = Schema {
            fields: fields1,
            nullable: false,
        };
        let schema2 = Schema {
            fields: fields2,
            nullable: false,
        };

        let diff = registry.diff_schemas(&schema1, &schema2);
        assert!(diff.added.contains(&"new_field".to_string()));
        assert!(diff.removed.is_empty());
        assert!(diff.changed.is_empty());
    }

    #[test]
    fn test_schema_diff_detects_removed() {
        let registry = SchemaRegistry::new();

        let mut fields1 = HashMap::new();
        fields1.insert(
            "old_field".to_string(),
            FieldDef {
                type_name: "i64".to_string(),
                optional: false,
                deprecated: true,
                version_added: Some("1.0".to_string()),
                version_deprecated: Some("1.1".to_string()),
            },
        );

        let fields2 = HashMap::new();

        let schema1 = Schema {
            fields: fields1,
            nullable: false,
        };
        let schema2 = Schema {
            fields: fields2,
            nullable: false,
        };

        let diff = registry.diff_schemas(&schema1, &schema2);
        assert!(diff.removed.contains(&"old_field".to_string()));
    }

    #[test]
    fn test_schema_diff_detects_changed() {
        let registry = SchemaRegistry::new();

        let mut fields1 = HashMap::new();
        fields1.insert(
            "field".to_string(),
            FieldDef {
                type_name: "i64".to_string(),
                optional: false,
                deprecated: false,
                version_added: None,
                version_deprecated: None,
            },
        );

        let mut fields2 = HashMap::new();
        fields2.insert(
            "field".to_string(),
            FieldDef {
                type_name: "string".to_string(),
                optional: true,
                deprecated: false,
                version_added: None,
                version_deprecated: None,
            },
        );

        let schema1 = Schema {
            fields: fields1,
            nullable: false,
        };
        let schema2 = Schema {
            fields: fields2,
            nullable: false,
        };

        let diff = registry.diff_schemas(&schema1, &schema2);
        assert!(diff.changed.contains(&"field".to_string()));
    }

    #[test]
    fn test_allow_runtime_field_insertion() {
        let registry = SchemaRegistry::new();

        let schema = Schema {
            fields: HashMap::new(),
            nullable: false,
        };

        // Should succeed for new field
        assert!(registry
            .allow_runtime_field_insertion(&schema, "new_field")
            .is_ok());

        // Should fail for existing field
        let mut fields = HashMap::new();
        fields.insert(
            "existing".to_string(),
            FieldDef {
                type_name: "string".to_string(),
                optional: true,
                deprecated: false,
                version_added: None,
                version_deprecated: None,
            },
        );

        let schema_with_field = Schema {
            fields,
            nullable: false,
        };

        assert!(registry
            .allow_runtime_field_insertion(&schema_with_field, "existing")
            .is_err());
    }
}

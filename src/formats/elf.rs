use crate::errors::Result;
use object::Object;

pub struct ElfAnalyzer<'a> {
    file: &'a object::File<'a>,
}

impl<'a> ElfAnalyzer<'a> {
    pub fn new(file: &'a object::File<'a>) -> Self {
        Self { file }
    }

    pub fn extract_metadata(&self) -> Result<serde_json::Value> {
        let mut metadata = serde_json::Value::Object(serde_json::Map::new());

        if let Some(obj) = metadata.as_object_mut() {
            obj.insert(
                "type".to_string(),
                serde_json::Value::String("ELF".to_string()),
            );
            obj.insert(
                "section_count".to_string(),
                serde_json::Value::Number(self.file.sections().count().into()),
            );
            obj.insert(
                "symbol_count".to_string(),
                serde_json::Value::Number(self.file.symbols().count().into()),
            );
        }

        Ok(metadata)
    }
}

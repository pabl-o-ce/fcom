use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct CombinerTemplate {
    template: String,
}

impl CombinerTemplate {
    pub fn from_string(template_string: &str) -> Self {
        CombinerTemplate {
            template: template_string.to_string(),
        }
    }

    pub fn from_file(template_file: &Path) -> Self {
        let template_string: String =
            fs::read_to_string(template_file).expect("Unable to read template file");
        CombinerTemplate {
            template: template_string,
        }
    }

    pub fn generate_output_file_content(&self, template_fields: &HashMap<&str, String>) -> String {
        let mut output = self.template.clone();
        for (key, value) in template_fields {
            output = output.replace(&format!("{{{}}}", key), value);
        }
        output
    }
}

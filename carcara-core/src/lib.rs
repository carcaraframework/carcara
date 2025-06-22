pub mod spec;

use spec::Carcarafile;
use anyhow::{Context, Result};
use std::fs;

pub fn parse_carcarafile(path: &str) -> Result<Carcarafile> {
    let file_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file at '{}'", path))?;
    
    let carcarafile: Carcarafile = toml::from_str(&file_content)
        .with_context(|| format!("Failed to parse TOML content from '{}'", path))?;
    
    Ok(carcarafile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec::{Action, ExecutionBlock, Step, UrlParams};

    #[test]
    fn it_parses_a_simple_toml_file() {
        let test_file = "test.carcara";
        
        let content = r#"
# TOML uses an "array of tables" to represent a list of objects.
[[step]]
id = "navigate-to-login"
description = "Fly to the login page"
# The action is a sub-table within the step table
[step.flyto]
url = "https://example.com"
"#;
        fs::write(test_file, content).unwrap();

        let result = parse_carcarafile(test_file);
        assert!(result.is_ok(), "Parsing failed when it should have succeeded.");
        
        let carcarafile = result.unwrap();
        assert_eq!(carcarafile.0.len(), 1, "Should parse one execution block.");
        
        if let Some(ExecutionBlock::Step(step)) = carcarafile.0.get(0) {
            assert_eq!(step.id, Some("navigate-to-login".to_string()));
            if let Action::Flyto(params) = &step.action {
                assert_eq!(params.url, "https://example.com");
            } else {
                panic!("Action should be Flyto");
            }
        } else {
            panic!("Execution block should be a Step");
        }

        fs::remove_file(test_file).unwrap();
    }
}

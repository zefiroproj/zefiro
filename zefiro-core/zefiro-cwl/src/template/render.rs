use anyhow::Error;
use serde_json::Value;
use tera::{Context, Tera};

pub struct TemplateRender {
    content: Value,
    tera: Tera,
}

impl TemplateRender {
    pub fn new(content: Value, template: &str) -> Result<Self, Error> {
        let mut tera = Tera::default();
        tera.add_raw_template("template", template)?;
        Ok(Self { content, tera })
    }

    pub fn render(&self) -> Result<String, Error> {
        let mut context = Context::new();
        if let Some(object) = self.content.as_object() {
            for (key, value) in object {
                context.insert(key, value);
            }
        }
        let result = self.tera.render("template", &context)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case(
        json!({"inputLocation": "s3://bucket"}),
        r#"
        input_file:
            class: location
            location: {{ inputLocation }}/dir/input.txt
        "#,
        r#"
        input_file:
            class: location
            location: s3://bucket/dir/input.txt
        "#,
    )]
    #[case(
        json!({
            "inputLocation": "s3://bucket",
            "suffixes": [1, 2, 3]
        }),
        r#"
        input_file:
        {% for suffix in suffixes %}
        - class: File
          location: {{ inputLocation }}/dir/input-{{ suffix }}.txt
        {% endfor %}
        "#,
        r#"
        input_file:
        
        - class: File
          location: s3://bucket/dir/input-1.txt
        
        - class: File
          location: s3://bucket/dir/input-2.txt
        
        - class: File
          location: s3://bucket/dir/input-3.txt
        
        "#,
    )]
    fn test_render(#[case] content: Value, #[case] template: &str, #[case] expected: &str) {
        let template_render = TemplateRender::new(content.clone(), template).unwrap();
        let rendered = template_render.render().unwrap();
        assert_eq!(rendered, expected);
    }
}

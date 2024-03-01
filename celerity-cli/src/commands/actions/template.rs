use core::projects::types::template::Template;
use comfy_table::Table;

pub struct TemplateAction;

impl TemplateAction {
    pub fn list(table: &mut Table){
        let templates = Template::template_list(String::from("examples/templates"));
        let template_length = templates.len();
        table.set_header(vec!["Name", "Author"]);
        for template in templates.into_iter().flatten() {
            table.add_row(vec![
                template.name,
                template.author
            ]);
        }
        println!("Templates list ({})\n{}", template_length, table)
    }
}
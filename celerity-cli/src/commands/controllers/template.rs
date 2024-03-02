use core::services::template::TemplateService;

use comfy_table::Table;

pub struct TemplateController;

impl TemplateController {
    pub fn show_templates(template_path: String, table: &mut Table){
        let list = TemplateService::list(template_path);
        if list.is_empty() {
            println!("Your template list is empty!");
        }
        table.set_header(vec!["Name", "Author", "Source", "Description"]);
        for template in  list.iter().flatten() {
            table.add_row(vec![
                template.name.to_owned(),
                template.author.to_owned(),
                template.path.uri.to_owned(),
                template.description.to_owned()
            ]);
        }
        println!("Templates ({}):\n{}", list.len(), table);
    }
}
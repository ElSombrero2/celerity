use core::projects::template::list;

use comfy_table::Table;

pub fn show_template_list(table: &mut Table){
    let templates = list(String::from("examples/templates"));
    let template_length = templates.len();
    table.set_header(vec!["Name", "Author"]);
    for template in templates {
        if let Some(template) = template {
            table.add_row(vec![
                template.name,
                template.author
            ]);
        }
    }
    println!("Templates list ({})\n{}", template_length, table)
}
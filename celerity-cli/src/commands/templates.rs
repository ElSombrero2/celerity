use core::projects::template_list;

use comfy_table::Table;

pub fn show_template_list(table: &mut Table){
    let templates = template_list(String::from("examples/templates"));
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
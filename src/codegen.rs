use nom::IResult;

use crate::Widget;

const STATELES_WIDGET_TEMPLATE: &str = "
import 'package:flutter/material.dart';

class {name} extends {type} {
  {positional_attributes}
  {named_attributes}
  {name}({constructor_signature});
}
";

const STATEFUL_WIDGET_TEMPLATE: &str = "
import 'package:flutter/material.dart';

class {name} extends {type} {
  {positional_attributes}
  {named_attributes}
  {name}({constructor_signature});
}
  
class {name}State extends State<{name}> {
  {states}
}
";

fn get_widget_type(widget: &Widget) -> String {
    let mut widget_type = String::from("StatelessWidget");

    if widget.states.len() > 0 {
        widget_type = String::from("StatefulWidget");
    }

    widget_type
}

pub fn generate_code(widget: Widget) -> IResult<(), String> {
    let mut code = String::new();

    let widget_type = get_widget_type(&widget);

    let positional_attributes: String = widget
        .positional_attributes
        .iter()
        .fold(String::new(), |acc, attribute| {
            format!("{}{} {};\n", acc, attribute.a_type, attribute.name)
        });

    let named_attributes: String = widget
        .named_attributes
        .iter()
        .fold(String::new(), |acc, attribute| {
            format!("{}{} {};\n", acc, attribute.a_type, attribute.name)
        });

    let states: String = widget.states.iter().fold(String::new(), |acc, state| {
        format!("{}{} {};\n", acc, state.s_type, state.name)
    });

    let positional_attributes_list: String = widget
        .positional_attributes
        .iter()
        .map(|attribute| format!("this.{}", attribute.name.to_string()))
        .collect::<Vec<String>>()
        .join(",");

    let named_attributes_list: String = widget
        .named_attributes
        .iter()
        .map(|attribute| format!("required this.{}", attribute.name.to_string()))
        .collect::<Vec<String>>()
        .join(",");

    let mut template = String::new();
    if widget_type == "StatelessWidget" {
        template = STATELES_WIDGET_TEMPLATE.to_string();
    } else if widget_type == "StatefulWidget" {
        template = STATEFUL_WIDGET_TEMPLATE.to_string();
    }

    let mut constructor_signature_template = String::new();
    if widget.positional_attributes.len() > 0 {
        constructor_signature_template = format!("{},", &positional_attributes_list)
    }
    if widget.named_attributes.len() > 0 {
        constructor_signature_template = format!(
            "{}{{{}}}",
            constructor_signature_template, &named_attributes_list
        );
    }

    template = template.replace("{name}", &widget.name);
    template = template.replace("{type}", &widget_type);
    template = template.replace("{positional_attributes}", &positional_attributes);
    template = template.replace("{named_attributes}", &named_attributes);
    template = template.replace("{states}", &states);
    template = template.replace("{constructor_signature}", &constructor_signature_template);

    code.push_str(&template);

    Ok(((), code))
}

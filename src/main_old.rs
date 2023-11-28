use pest::{error::Error, Parser};
use pest_derive::Parser;
use std::{
    fmt::{self, write},
    fs,
};

#[derive(Parser)]
#[grammar = "arrow.pest"]
pub struct ArrowParser;

fn main() {
    let unparsed_file = fs::read_to_string("./examples/partial.arrow").expect("cannot read file");

    let file = parse(&unparsed_file).expect("unsuccessful parse");

    println!("result: {:?}", file.to_string());
}

enum WidgetType {
    Stateless,
    Stateful,
}

pub struct Widget {
    name: String,
    wtype: WidgetType,
    rest: String,
    states: Vec<String>,
    reads: Vec<String>,
    watches: Vec<String>,
    selects: Vec<String>,
    attributes: Vec<String>,
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let widget_type = match self.wtype {
            WidgetType::Stateless => "StatelessWidget",
            WidgetType::Stateful => "StatefulWidget",
        };

        print!("class {} extends {}\n", self.name, widget_type);
        write!(f, "Rest >>> {}", self.rest)
    }
}

pub fn parse(source: &str) -> Result<Widget, Error<Rule>> {
    let mut widget = Widget {
        name: String::new(),
        wtype: WidgetType::Stateless,
        rest: String::new(),
        states: Vec::new(),
        reads: Vec::new(),
        watches: Vec::new(),
        selects: Vec::new(),
        attributes: Vec::new(),
    };

    let pairs = ArrowParser::parse(Rule::program, source)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::widget => {
                let mut thispair = pair.into_inner();
                let widget_name = thispair.next().unwrap();
                widget.name = widget_name.as_str().to_string();

                // let widget_type = thispair.next().unwrap();

                // widget.rest.push_str(widget_type.as_str());
            }
            Rule::body => {
                let x = pair.into_inner().next().unwrap();
                print!("body: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::typeIdent => {
                let x = pair.into_inner().next().unwrap();
                print!("typeIdent: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::attribute => {
                let mut thispair = pair.into_inner();
                let attribute_name = thispair.next().unwrap();
                // let attribute_value = thispair.next().unwrap();

                print!("attribute: {}", attribute_name);
            }
            Rule::ident => {
                let x = pair.into_inner().next().unwrap();
                print!("ident: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::stateField => {
                let mut thispair = pair.into_inner();
                let state_name = thispair.next().unwrap();
                // let state_value = thispair.next().unwrap();

                print!("stateField: {}", state_name);
            }
            Rule::attributeScope => {
                let mut thispair = pair.into_inner();
                let attribute_scope_name = thispair.next().unwrap();
                // let attribute_scope_value = thispair.next().unwrap();

                // print!("attributeScope: {}", attribute_scope_name);
                widget.rest.push_str(attribute_scope_name.as_str());
            }
            Rule::field => {
                let mut thispair = pair.into_inner();
                let field_name = thispair.next().unwrap();
                // let field_value = thispair.next().unwrap();

                print!("field: {}", field_name);
                widget.rest.push_str(field_name.as_str());
            }
            Rule::lineFeed => {
                let x = pair.into_inner().next().unwrap();
                print!("lineFeed: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::stateFieldType => {
                let x = pair.into_inner().next().unwrap();
                print!("stateFieldType: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::providerFieldType => {
                let x = pair.into_inner().next().unwrap();
                print!("providerFieldType: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::providerField => {
                let x = pair.into_inner().next().unwrap();
                print!("providerField: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::selectFieldType => {
                let x = pair.into_inner().next().unwrap();
                print!("selectFieldType: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::selectRename => {
                let x = pair.into_inner().next().unwrap();
                print!("selectRename: {}", x);
                widget.rest.push_str(x.as_str());
            }
            Rule::selectField => {
                let x = pair.into_inner().next().unwrap();
                print!("selectField: {}", x);
                widget.rest.push_str(x.as_str());
            }
            _ => {
                // ast.push_str(&format!("other: {}\n", pair.as_str()));
            }
        }
    }

    Ok(widget)
}

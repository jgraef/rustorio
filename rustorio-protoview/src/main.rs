use std::fmt::{
    self,
    Display,
    Formatter,
};

use cursive::{
    direction::Direction,
    event::{
        Event,
        EventResult,
    },
    traits::View,
    view::{
        CannotFocus,
        Nameable,
        Resizable,
    },
    views::Dialog,
    Cursive,
    CursiveExt,
    Printer,
    Vec2,
};
use cursive_tree_view::{
    Placement,
    TreeView,
};
use rustorio_core::{
    error::Error,
    mod_loader::ModLoader,
};
use rustorio_data::value::{
    Key,
    Table,
    Value,
};

#[derive(Debug)]
enum Item {
    Root,
    Node(Key),
    Leaf(Key, Value), // Value can't be a Table
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Item::Root => write!(f, "data.raw"),
            Item::Node(key) => write!(f, "{:?}: Table", key),
            Item::Leaf(key, value) => {
                write!(
                    f,
                    "{:?}: {:?} = {:?}: {:?}",
                    key,
                    key.ty(),
                    value,
                    value.ty()
                )
            }
        }
    }
}

struct PrototypeTree {
    view: TreeView<Item>,
}

impl Default for PrototypeTree {
    fn default() -> Self {
        Self {
            view: TreeView::new(),
        }
    }
}

impl PrototypeTree {
    pub fn fill(&mut self, data: Value) {
        match data {
            Value::Table(root) => {
                let row = self
                    .view
                    .insert_item(Item::Root, Placement::LastChild, 0)
                    .unwrap();
                self.fill_children(root, row, 1)
            }
            _ => panic!("Root must be a table"),
        }
    }

    fn fill_children(&mut self, table: Table, parent: usize, depth: usize) {
        if depth > 4 {
            return;
        }

        for (key, value) in table {
            match value {
                Value::Table(table) => {
                    let row = self
                        .view
                        .insert_item(Item::Node(key), Placement::LastChild, parent)
                        .unwrap();
                    self.fill_children(table, row, depth + 1);
                    self.view.collapse_item(row);
                }
                value => {
                    self.view
                        .insert_item(Item::Leaf(key, value), Placement::LastChild, parent);
                }
            }
        }
    }
}

impl View for PrototypeTree {
    fn draw(&self, printer: &Printer) {
        self.view.draw(printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.view.layout(size)
    }

    fn required_size(&mut self, req: Vec2) -> Vec2 {
        self.view.required_size(req)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        self.view.on_event(event)
    }

    fn take_focus(&mut self, direction: Direction) -> Result<EventResult, CannotFocus> {
        self.view.take_focus(direction)
    }
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let mut loader = ModLoader::new_with_base("data/core", "data/base")?;
    loader.check_dependencies()?;

    let data_raw: Value = loader.data_stage()?;

    /* TODO: Remove again. I just botched this to export all signals
    match &data_raw {
        Value::Table(table) => {
            for (k, v) in table.iter() {
                let ty = match k {
                    Key::String(s) => s,
                    _ => panic!(),
                };

                match v {
                    Value::Table(table) => {
                        if ty == "item" || ty == "fluid" || ty == "virtual-signal" {
                            let ty = match ty.as_str() {
                                "item" => "SignalType::Item",
                                "fluid" => "SignalType::Fluid",
                                "virtual-signal" => "SignalType::Virtual",
                                _ => panic!(),
                            };

                            for (k2, _) in table.iter() {
                                let name = match k2 {
                                    Key::String(s) => s,
                                    _ => panic!(),
                                };

                                println!("SignalID::new({}, {:?}.to_owned()),", ty, name);
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
        _ => {},
    }
    panic!("foo");
    */

    /*let data_raw = Value::Table(
        vec![
            (Key::String("1st_child".to_owned()), Value::Nil),
            (Key::String("2nd_child".to_owned()), Value::Boolean(true)),
            (Key::String("3rd_child".to_owned()), Value::String("foobar".to_owned())),
            (Key::String("4th_child".to_owned()), Value::Integer(420)),
            (Key::String("5th_child".to_owned()), Value::Number(42.0)),
        ]
        .into_iter()
        .collect(),
    );*/

    let mut siv = Cursive::default();

    let mut tree = PrototypeTree::default();
    tree.fill(data_raw);

    siv.add_layer(
        Dialog::around(tree.with_name("tree"))
            .title("data.raw")
            .full_screen(),
    );

    siv.run();

    Ok(())
}

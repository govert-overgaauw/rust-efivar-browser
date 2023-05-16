use cursive::traits::*;
use cursive::views::{Dialog, SelectView};

use efivar::*;

fn main() {
    let mut select: SelectView = SelectView::new();

    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let var_man = system();
    let enumerator: Box<dyn VarManager> = var_man;
    for var in enumerator
        .get_var_names()
        .expect("failed to list variable names!")
    {
        let text: String = var.short_name();
        println!("{}", text);
        select = select.item_str(text);
    }

    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10))).title("Choose a variable"),
    );
    // Starts the event loop.
    siv.run();
}

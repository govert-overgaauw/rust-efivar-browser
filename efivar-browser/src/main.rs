use cursive::views::{Dialog, TextView};
use efivar::*;

fn main() {
	let var_man = system();
	let enumerator: Box<dyn VarManager> = var_man;
	for var in enumerator.get_var_names().expect("failed to list variable names!")
	{
		println!("{}", var.short_name());
	}

	// Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
                         .title("Cursive")
                         .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}

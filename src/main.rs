mod editor;

use editor::Editor;

fn  main() -> Result<(), std::io::Error> {
    let mut editor: Editor = Editor::new()?;
    editor.run()
}
use std::io::{self, Read, Write};
use std::io::{Stdin, Bytes};
use termion::raw::RawTerminal;
use termion::raw::IntoRawMode;


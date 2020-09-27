`just_text` is a tiny tool for organizing any kind of text-based writing.
Here is all you need to do to get started:

1. Add .txt files to a directory called _notes_
2. Execute `just_text` from within the parent directory

This will generate a _build_ directory containing an index with links to each note, and an .html file for each note itself. A note's title is determined by its filename: _My_New_Note.txt_ will have the title "My New Note".

## Installation

At the moment, the only way to install `just_text` is to clone this repository and execute `cargo install --path <path-to-repo>`.

## Additional Details

After the first run, `just_text` will generate and update a file containing metadata for each note in a file called `.notes`.
At the moment, this is only used to establish the creation date for each note, but I may extend it to include a "last updated" date, a hash of the most recent contents, etc.

## Planned improvements

-   Properly escape all characters that might cause issues (at the moment, I have only done so for double quotes).
-   Look for opportunities to borrow `&str`s instead of allocating.
-   Investigate tradeoffs of generating notes in separate threads.

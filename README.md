A small project to let you see your directory structure, should be cross platform!

To get set up
''' sh
git clone https://github.com/ajassi9751/dir-explorer
cd dir-explorer
cargo build
'''

To run
''' sh
cargo run
'''

I plan to add the ability to jump to a specific file or directory depending on the flags given to the program.
It will also fuzzy find when you are jumping.

I don't use rayon anymore because the parralel iterator prints in random order (the code lives in the rayon branch).

# Some static blog thing, idk

From what I know, the lack of a license on this repo means all rights reserved.
I like that. Please don't use this, it's mines and it is bad.

This reads in pages from `pages/` with the `.md` extension to display as HTML pages.
If you have a matching filename in `songs/` with the `.song` extension, it will load the URL from it (content should only be a url, no quotes etc) and try and display a Spotify embedded player.

It's rather tailored to my needs so you probably won't like it.

## Setup

```sh
# Build the binary
cargo build

# make a quick symlink to it, since i update a lot
ln -s target/debug/abstractumbra-site app

# make the executable executable
chmod u+x target/debug/abstractumbra-site

# and run
./app
```

The index (`/`) will show all pages in `pages/` that aren't prefixed with `.` in the filename.

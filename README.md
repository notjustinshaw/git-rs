# git.rs
A reimplementation of git internals in Rust (compatable with Git).

## Description
In an effort to better understand how git works (and also to learn Rust), I decided to write my own git in Rust. This repository is still a work in progress, since I don't intend to make it feature-complete with git. Though, it might make it into a blog post at some point so I decided to leave it around. I figure others might find it interesting to fiddle around with so I'll just keep adding documentation to this readme as I write the code.

## A Short Tour

```
.
├── src                     # The application code is here
│   ├── cli/                  # Handles the command-line interface
│   ├── crypto/               # Handles compression and hashing
│   ├── object/               # Handles git-objects and their representations (commit, blob, ref, etc.)
│   ├── repo/                 # Handles repository metadata (working tree, configs, etc.)
│   └── main.rs               # The entrypoint of the appliation
└── test                    # The testing code is here
    └── ...                   # Testing code is in here
```

### Git Internals
At its core, git is a command-line utility for tracking changes to a directory in a decentralized manner. Logically, there is a git repository that acts like a tree which steps forward in time from one commit to another commit, each time only tracking the changes (ie. diffs) from one revision to the next. Here, we will try to rebuild a git from the core components in a way that will be backwards compatable with git itself. Stay tuned for more!

See also: https://wyag.thb.lt/

## License
MIT

# git.rs
the rusty content tracker

## Terminology
`<object>`

Indicates the object name for any type of object

`<blob>`

Indicates a blob object name.

`<tree>`

Indicates a tree object name.

`<commit>`

Indicates a commit object name.

`<tree-ish>`

Indicates a tree, commit or tag object name. A command that takes a <tree-ish> argument ultimately wants to operate on a <tree> object but automatically dereferences <commit> and <tag> objects that point at a <tree>.

`<commit-ish>`

Indicates a commit or tag object name. A command that takes a <commit-ish> argument ultimately wants to operate on a <commit> object but automatically dereferences <tag> objects that point at a <commit>.

`<type>`

Indicates that an object type is required. Currently one of: blob, tree, commit, or tag.

`<file>`

Indicates a filename - almost always relative to the root of the tree structure GIT_INDEX_FILE describes.

## Symbolic Identifiers
Any Git command accepting any <object> can also use the following symbolic notation:

`HEAD`

indicates the head of the current branch.

`<tag>`

a valid tag name (i.e. a `refs/tags/<tag>` reference).

`<head>`

a valid head name (i.e. a `refs/heads/<head>` reference).
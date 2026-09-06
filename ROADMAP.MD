# Roadmap

Desired features:

- Enforce a naming convention
- Resolve duplicate names
  - Implement a post-fix system to avoid data loss
- Show changes before the convention is enforced
- Ignore files on .gitignore
- Implement a file-based configuration
- Set a minimum and maximum length for files and directories names

## Step one

Implement a system that reads a directory and scans for all the child files and directories, detects whether or not it already has a naming convention, and then rename the files to apply the kebab-case convention.  
Implement pretty print  
Skip files that already have kebab-case name

## Step two

Resolve name conflicts (Duplications)  
Find and document as many edge cases as possible

## Step three

Add configurations  
Recursively search for a `bireme.toml` file (upwards)
Enable recursive mode  
Ignore dotfiles  
Scan .gitignore file

## Step four

Add more conventions

## Step five

Publish to crates.io

## Considerations

- Symlinks may create errors
- Prevent working on the root directory
- Some names will be impossible to parse, decide how to handle such cases

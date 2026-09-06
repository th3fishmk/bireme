# Bireme

This tool enforces a naming convention on a given directory

> **Warning** The current state of this program allow for data loss! DO NOT RUN THIS PROGRAM IF YOU HAVE IMPORTANT INFORMATION IN THE TARGET DIRECTORY

```bash
bireme <target_dir>

# Examples:

bireme ~/Documents/ # Fix your document directory
bireme src/ # Fix your src directory
bireme init/ # Fix a directory named init
```

Will scan and rename all the files and directories in the current directory

## Features

Scan and rename your files so they look nice and clean:

Before

```plaintext
documents/
├── Final Report (v2) - Draft.pdf
├── Financials 2026/
│   ├── Q1_SUMMARY--FINAL.xlsx
│   └── Q2 expense report (OCTOBER).csv
├── IMPORTANT README.txt
├── my Resume (Updated Version!).docx
├── NEW_DESIGN_mockup [v1].png
└── ~temp file #1.tmp
```

After

```plaintext
documents/
├── final-report-v2-draft.pdf
├── financials-2026/
│   ├── q1-summary-final.xlsx
│   └── q2-expense-report-october.csv
├── important-readme.txt
├── my-resume-updated-version.docx
├── new-design-mockup-v1.png
└── temp-file-1.tmp
```

## Configuration

This tool is customizable, check the [configuration file](./bireme.toml) to see all the configurations avaible, their defaults, and their options. To change these settings, you must create a copy of this file by running `bireme init` (or manually create a file named `bireme.toml`) and do your customizations.

# Bireme

This tool enforces a naming convention on a given directory

```bash
$ bireme <target_dir>
# Examples:
$ bireme # Run in the current directory
$ bireme ~/Documents/ # Run in the Documents directory
$ bireme src/ # Run in the src
```

## Features

Scan and rename your files and directories so they look nice and clean:

Before

```plaintext
$ tree
.
├── Final Report (v2) - Draft.pdf
├── Financials 2026
│   ├── Q1_SUMMARY--FINAL.xlsx
│   └── Q2 expense report (OCTOBER).csv
├── IMPORTANT README.txt
├── NEW_DESIGN_mockup [v1].png
├── Vacation 2026
│   ├── IMG_9920 [edited].jpeg
│   ├── beach photo (1).jpg
│   ├── family & friends #2.jpg
│   ├── sunset_at_hotel.png
│   └── ~DSC_0001 (RAW).CR2
├── my Resume (Updated Version!).docx
└── ~temp file #1.tmp

3 directories, 12 files
```

After

```plaintext
$ tree
.
├── final-report-v2-draft.pdf
├── financials-2026
│   ├── q1-summary-final.xlsx
│   └── q2-expense-report-october.csv
├── important-readme.txt
├── my-resume-updated-version.docx
├── new-design-mockup-v1.png
├── temp-file-1.tmp
└── vacation-2026
    ├── beach-photo-1.jpg
    ├── dsc-0001-raw.cr2
    ├── family-friends-2.jpg
    ├── img-9920-edited.jpeg
    └── sunset-at-hotel.png

3 directories, 12 files
```

<!--## Configuration

This tool is customizable, check the [configuration file](./bireme.toml) to see all the configurations avaible, their defaults, and their options. To change these settings, you must create a copy of this file by running `bireme init` (or manually create a file named `bireme.toml`) and do your customizations.-->

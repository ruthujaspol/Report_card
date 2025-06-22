# 📝 Rust Report Card App

A simple command-line app built in Rust to generate a student report card with calculated average and grade.

## 🔧 Features

- Accepts:
  - Student name
  - Total marks
  - Number of subjects
- Calculates average using a custom function
- Assigns grades based on:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: Below 60
- Prints the report card in terminal
- Saves the report in `report_card.txt`
- Includes a `report_card.pdf` file exported from the `.txt`

## 📂 Files Included

- `src/main.rs` – Rust code
- `report_card.txt` – Text version of report card
- `report_card.pdf` – Exported PDF version
- `Cargo.toml` – Project manifest

## 📸 Sample Output

📝 Report Card 📝

Name: Rutuja Pol <br />
Total Marks: 450 <br />
Subjects: 5 <br />
Average: 90.00 <br />
Grade: A

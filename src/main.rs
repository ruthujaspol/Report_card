use std::io;
use std::fs::File;
use std::io::Write;

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(avg: f32) -> &'static str {
    match avg {
        x if x >= 90.0 => "A",
        x if x >= 75.0 => "B",
        x if x >= 60.0 => "C",
        _ => "D",
    }
}

fn main() {
    let mut name = String::new();
    let mut total = String::new();
    let mut subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total).unwrap();
    let total_marks: f32 = total.trim().parse().unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects).unwrap();
    let num_subjects: u32 = subjects.trim().parse().unwrap();

    let avg = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(avg);

    println!("\n📝 Report Card 📝");
    println!("Name: {}", name.trim());
    println!("Total Marks: {}", total_marks);
    println!("Subjects: {}", num_subjects);
    println!("Average: {:.2}", avg);
    println!("Grade: {}", grade);

    // Save as text file
    let report = format!(
        "📝 Report Card 📝\n\nName: {}\nTotal Marks: {}\nSubjects: {}\nAverage: {:.2}\nGrade: {}",
        name.trim(), total_marks, num_subjects, avg, grade
    );

    let mut file = File::create("report_card.txt").unwrap();
    file.write_all(report.as_bytes()).unwrap();

    println!("\n✅ Report saved as report_card.txt");
}

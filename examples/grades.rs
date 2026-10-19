// grades.rs
// Coding test: given a list of students and their test scores, print each
// student's average and letter grade, then the student with the best average.

struct Student {
    name: String,
    scores: Vec<i64>,
}

fn make_student(name: &str, scores: &[i64]) -> Student {
    Student {
        name: name.to_string(),
        scores: scores.to_vec(),
    }
}

fn average(scores: &[i64]) -> i64 {
    if scores.is_empty() {
        return 0;
    }

    scores.iter().sum::<i64>() / scores.len() as i64
}

fn letter_grade(score: i64) -> &'static str {
    if score >= 90 {
        return "A";
    }
    if score >= 80 {
        return "B";
    }
    if score >= 70 {
        return "C";
    }

    "F"
}

fn main() {
    let students = vec![
        make_student("Alice", &[92, 88, 95]),
        make_student("Bob", &[70, 65, 81]),
        make_student("Cara", &[85, 90, 79]),
        make_student("Dan", &[58, 64, 61]),
    ];

    let mut top_name = "";
    let mut top_average = 0;
    for student in &students {
        let avg = average(&student.scores);
        let grade = letter_grade(avg);
        println!("{}: {} {}", student.name, avg, grade);

        if avg > top_average {
            top_average = avg;
            top_name = &student.name;
        }
    }

    println!("Top student: {}", top_name);
}

trait Course {
    fn get_overview(&self) -> String;
}

struct Workshop {
    title: String,
    instructore: String,
    duration: String,
}

struct Seminar {
    title: String,
    speaker: String,
    location: String,
}

// Here we used `format!()`, it returns the string
impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!(
            "Workshop Title: {}, Workshop Instructor: {}, Workshop Duration: {}",
            self.title, self.instructore, self.duration
        )
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!(
            "Seminar Title: {}, Seminar Speaker: {}, Seminar Location: {}",
            self.title, self.speaker, self.location
        )
    }
}

fn print_course_overview<T: Course>(param: T) {
    println!("{}", param.get_overview());
}

fn main() {
    let workshop: Workshop = Workshop {
        title: "Blockchain".to_owned(),
        instructore: "Alice".to_owned(),
        duration: "4 Hours".to_owned(),
    };

    let seminar: Seminar = Seminar {
        title: "Bitcoin: The Digital Gold".to_owned(),
        speaker: "Shoaib".to_owned(),
        location: "Delhi, India".to_owned(),
    };

    // call the functions
    // workshop.get_overview();
    // seminar.get_overview();

    // println!("Workshop Details -> {}", workshop.get_overview());
    // println!("Seminar Details -> {}", seminar.get_overview());

    print_course_overview(workshop);
    print_course_overview(seminar);
}

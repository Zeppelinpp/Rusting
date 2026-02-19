struct Musician {
    name: String,
    email: String,
    genera: String,
    status: Status,
}

struct Touring {
    city: String,
    date: String,
    is_sold_out: bool,
}

#[allow(dead_code)]
enum Status {
    Active(bool),
    NotActive(bool),
    OnTour(Touring),
}

impl Touring {
    fn get_info(&self) -> String {
        format!(
            "City: {}\nDate: {}\n{}",
            self.city,
            self.date,
            if self.is_sold_out {
                "No tickets available"
            } else {
                "Tickets available"
            }
        )
    }
}

impl Musician {
    fn desc(&self) {
        println!("====================");
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("general {}", self.genera);
        println!("====================");
    }

    fn set_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    fn set_status(&mut self, status: Status) {
        match &status {
            Status::Active(val) => {
                println!("Setting active with {}", val);
            }
            Status::NotActive(val) => {
                println!("Setting active with {}", val);
            }
            Status::OnTour(t) => {
                let touring_info = t.get_info();
                println!("Setting Touring:\n{}", touring_info);
            }
        }
        self.status = status;
    }
    fn check_tour(&self, city: &str) {
        if let Status::OnTour(touring) = &self.status {
            if touring.city == city {
                let touring_info = touring.get_info();
                println!("Currently touring:\n{}", touring_info,)
            } else {
                println!("No touring at {}", city)
            }
        } else {
            println!("No touring information")
        }
    }
}

fn main() {
    let kr_name = String::from("Kurt Rosenwinkel");
    let kr_email = format!("{}@gmail.com", kr_name.to_lowercase().replace(" ", ""));
    let mut kr = Musician {
        name: kr_name,
        email: kr_email,
        genera: String::from("Jazz/Modern Jazz"),
        status: Status::Active(true),
    };
    kr.desc();
    kr.set_email(String::from("newkurtrosenwinkel@gmail.com"));
    kr.set_status(Status::OnTour(Touring {
        city: String::from("Berlin"),
        date: String::from("2026-02-19"),
        is_sold_out: false,
    }));
    kr.desc();
    kr.check_tour("Berlin");
}

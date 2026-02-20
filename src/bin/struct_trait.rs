use std::any::type_name;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    DontCare(String),
}

#[derive(Debug)]
struct BasicInfo {
    name: String,
    age: i8,
    gender: Gender,
}

struct IndieMusician {
    info: BasicInfo,
    personality: String,
}

struct PopSinger {
    info: BasicInfo,
}

trait ReleaseAlbum {
    /// Every Human can ReleaseAlbum and has-a BasicInfo
    /// Compose with or without personality
    fn get_info(&self) -> &BasicInfo;

    fn compose(&self) {
        let yourself = type_name::<Self>();
        let short_name = yourself.split("::").last().unwrap_or(yourself);
        println!("--- {} composing ... ---", short_name);

        let info = self.get_info();
        println!(
            "Name: {}\nAge: {}\nGender: {}",
            info.name,
            info.age,
            match &info.gender {
                // Match with & reference won't take ownership
                Gender::Male => "Male",         // &str
                Gender::Female => "Female",     // &str
                Gender::DontCare(desc) => desc, // &String will Deref Coercion to &str
            }
        );
        self.apply_personality();
    }

    fn apply_personality(&self);
}

impl ReleaseAlbum for IndieMusician {
    fn get_info(&self) -> &BasicInfo {
        &self.info
    }
    fn apply_personality(&self) {
        println!("Applying {}...", self.personality);
        println!("--- Finish ---\n");
    }
}

impl ReleaseAlbum for PopSinger {
    fn get_info(&self) -> &BasicInfo {
        &self.info
    }
    fn apply_personality(&self) {
        println!("PopSinger have no personality, skip it. Making good money");
        println!("--- Finish ---\n");
    }
}

fn main() {
    let indie_musician = IndieMusician {
        info: BasicInfo {
            name: String::from("Tame Impala"),
            age: 30,
            gender: Gender::DontCare(String::from("maybe gay")),
        },
        personality: String::from("Indie/Neo-Psychedelia/Psychedelic Pop"),
    };
    let pop_singer_1 = PopSinger {
        info: BasicInfo {
            name: String::from("Taylor Swift"),
            age: 32,
            gender: Gender::Female,
        },
    };
    let pop_singer_2 = PopSinger {
        info: BasicInfo {
            name: String::from("Drake"),
            age: 36,
            gender: Gender::Male,
        },
    };

    indie_musician.compose();
    pop_singer_1.compose();
    pop_singer_2.compose();
}

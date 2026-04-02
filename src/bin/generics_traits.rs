use std::fmt;

trait Rating {
    type Score;
    fn score(&self) -> Self::Score;
}

struct RYM(f32);
impl Rating for RYM {
    type Score = f32;
    fn score(&self) -> f32 {
        self.0
    }
}
impl fmt::Display for RYM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/5.0 (RYM)", self.score())
    }
}

struct DownBeat(u8);
impl Rating for DownBeat {
    type Score = u8;
    fn score(&self) -> u8 {
        self.0
    }
}
impl fmt::Display for DownBeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/5 (DownBeat)", self.score())
    }
}

enum AlbumType {
    Physical,
    Digital,
    Streaming,
}

struct Album<R: Rating> {
    name: String,
    rating: R,
    album_type: AlbumType,
}
impl<R: Rating + fmt::Display> fmt::Display for Album<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let album_type = match self.album_type {
            AlbumType::Physical => "Physical",
            AlbumType::Digital => "Digital",
            AlbumType::Streaming => "Streaming",
        };

        write!(f, "[{}] {} - {}", album_type, self.name, self.rating)
    }
}

fn main() {
    let mut ok_computer = Album {
        name: "OK Computer".to_string(),
        rating: RYM(4.5),
        album_type: AlbumType::Digital,
    };

    let mut remedy = Album {
        name: "The Remedy(Live)".to_string(),
        rating: DownBeat(5),
        album_type: AlbumType::Physical,
    };

    let mut fall = Album {
        name: "Falling".to_string(),
        rating: DownBeat(3),
        album_type: AlbumType::Streaming,
    };

    println!("{}", ok_computer);
    println!("{}", remedy);
    println!("{}", fall);
}


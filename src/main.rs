use dotenv::dotenv;

struct Movie {
    Title: String,
    Genre: String,
    Description: String,
    Director: String,
    Actors: String,
    Year: String,
    Runtime: u8,
    Rating: f32,
    Votes: u32
}

struct Theatre {
    theatre_id: u32,
    name: String,
    city: String,
    state: String,
    theater_chain_name: String
}

struct Sales {
    theatre_id: u32,
    
}


fn main() {
    println!("Hello, world!");
}

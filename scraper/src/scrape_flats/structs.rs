use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Flat {
    description: String,
    street: String,
    rooms: i32,
    square: i32,
    floor: String,
    series: String,
    square_m_price: i32,
    price: i32,
}
impl Flat {
    // Create an empty Flat object
    pub fn new() -> Self {
        Flat {
            description: String::new(),
            street: String::new(),
            rooms: 0,
            square: 0,
            floor: String::new(),
            series: String::new(),
            square_m_price: 0,
            price: 0,
        }
    }

    // Setters for populating the fields
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_street(&mut self, street: String) {
        self.street = street;
    }

    pub fn set_rooms(&mut self, rooms: String) {
        let numeric_rooms = parse_string_to_numeric(rooms);
        self.rooms = numeric_rooms;
    }

    pub fn set_square(&mut self, square: String) {
        let numeric_square = parse_string_to_numeric(square);
        self.square = numeric_square;
    }

    pub fn set_floor(&mut self, floor: String) {
        self.floor = floor;
    }

    pub fn set_series(&mut self, series: String) {
        self.series = series;
    }

    pub fn set_square_m_price(&mut self, square_m_price: String) {
        let numeric_price = parse_string_to_numeric(square_m_price);
        self.square_m_price = numeric_price;
    }

    pub fn set_price(&mut self, price: String) {
        let numeric_price = parse_string_to_numeric(price);
        self.price = numeric_price;
    }
}

fn parse_string_to_numeric(value: String) -> i32 {
    let cleaned_input: String = value
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',')
        .collect();
    let parsed_int = cleaned_input.replace(",", "").parse::<i32>();
    let numeric_price = match parsed_int {
        Ok(price) => price,
        Err(_) => 0,
    };
    numeric_price
}

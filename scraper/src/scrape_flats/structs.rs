use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Flat {
    description: String,
    street: String,
    rooms: String,
    square: String,
    floor: String,
    series: String,
    square_m_price: String,
    price: String,
}
impl Flat {
    // Create an empty Flat object
    pub fn new() -> Self {
        Flat {
            description: String::new(),
            street: String::new(),
            rooms: String::new(),
            square: String::new(),
            floor: String::new(),
            series: String::new(),
            square_m_price: String::new(),
            price: String::new(),
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
        self.rooms = rooms;
    }

    pub fn set_square(&mut self, square: String) {
        self.square = square;
    }

    pub fn set_floor(&mut self, floor: String) {
        self.floor = floor;
    }

    pub fn set_series(&mut self, series: String) {
        self.series = series;
    }

    pub fn set_square_m_price(&mut self, square_m_price: String) {
        self.square_m_price = square_m_price;
    }

    pub fn set_price(&mut self, price: String) {
        self.price = price;
    }
}

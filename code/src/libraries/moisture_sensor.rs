pub fn get_reading(reading: u16) -> bool {
    if reading < 2000 {
        return false
    } else {
        return true
    }
}
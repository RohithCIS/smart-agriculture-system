pub fn get_reading(reading: u16) -> bool {
    if reading < 1750 {
        return false
    } else {
        return true
    }
}
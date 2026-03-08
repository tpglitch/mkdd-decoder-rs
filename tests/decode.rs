use mkdd_decoder::*;

#[test]
fn wrong_length_is_rejected() {
    assert_eq!(decode("TOOSHORT"), Err(DecodeError::WrongLength));
}

#[test]
fn bad_checksum_is_rejected() {
    assert_eq!(
        decode("GGGGGGGGGGGGGGGG"),
        Err(DecodeError::InvalidChecksum)
    );
}

#[test]
fn laptime_display() {
    assert_eq!(format!("{}", LapTime(524_287)), "08 : 44 . 287");
}

#[test]
fn course_display() {
    assert_eq!(format!("{}", Course::BowsersCastle), "Bowser's Castle");
}

#[test]
fn character_display() {
    assert_eq!(format!("{}", Character::PeteyPiranha), "Petey Piranha");
}

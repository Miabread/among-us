use among_us::EmergencyMeeting;

fn main() {
    let evidence = "RED SUS
BLUE SUS
WHO?
YELLOW SUS
GREEN SUS
WHERE?";
    EmergencyMeeting::from_evidence(evidence).execute();
}

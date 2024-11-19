fn main() {
    let my_name = "Blessed Sibanda";

    assert!(my_name == "Blessed Sibanda");
    assert_eq!(my_name, "Blessed Sibanda");
    assert_ne!(my_name, "John Doe");

    assert_ne!(
        my_name, "Blessed Sibanda",
        "You entered {my_name}. Input must not equal Blessed Sibanda"
    );
}

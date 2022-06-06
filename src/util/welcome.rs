use crate::util::user_input;

pub fn greet(user_choose: &mut String) {
    println!("Simple CLI number converter");
    println!("-------------------------------");
    
    decimal_option();
    binary_option();
    octal_option();
    hexa_option();

    print!("~> ");
    user_input::read(user_choose);
}

fn decimal_option() {
    print!(
        "
         1. Decimal --> Binary
         2. Decimal --> Octal
         3. Decimal --> Hexa
        "
    );
}

fn binary_option() {
    print!(
        "
         4. Binary   --> Decimal
         5. Binary   --> Octal
         6. Binary   --> Hexa
        "
    )
}

fn octal_option() {
    print!(
        "
         7. Octal   --> Decimal
         8. Octal   --> binary
         9. Octal   --> Hexa
        "
    )
}

fn hexa_option() {
    print!(
        "
        10. Hexa    --> Decimal
        11. Hexa    --> Binary
        12. Hexa    --> Octal
        "
    )
}
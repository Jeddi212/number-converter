use crate::util::user_input;

pub fn greet(user_choose: &mut String) {
    println!("Simple CLI number converter");
    println!("-------------------------------");
    
    decimal_option();
    biner_option();
    octal_option();
    hexa_option();

    print!("~> ");
    user_input::read(user_choose);
}

fn decimal_option() {
    print!(
        "
         1. Decimal --> Biner
         2. Decimal --> Octal
         3. Decimal --> Hexa
        "
    );
}

fn biner_option() {
    print!(
        "
         4. Biner   --> Decimal
         5. Biner   --> Octal
         6. Biner   --> Hexa
        "
    )
}

fn octal_option() {
    print!(
        "
         7. Octal   --> Decimal
         8. Octal   --> Biner
         9. Octal   --> Hexa
        "
    )
}

fn hexa_option() {
    print!(
        "
        10. Hexa    --> Decimal
        11. Hexa    --> Biner
        12. Hexa    --> Octal
        "
    )
}
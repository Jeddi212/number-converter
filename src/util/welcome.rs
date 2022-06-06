use crate::util::user_input;

pub fn greet(user_choose: &mut String) {
    println!("Simple CLI number converter");
    println!("-------------------------------");
    
    decimal_option();
    biner_option();
    oktal_option();
    hexa_option();

    print!("~> ");
    user_input::read(user_choose);
}

fn decimal_option() {
    print!(
        "
         1. Decimal --> Biner
         2. Decimal --> Oktal
         3. Decimal --> Hexa
        "
    );
}

fn biner_option() {
    print!(
        "
         4. Biner   --> Decimal
         5. Biner   --> Oktal
         6. Biner   --> Hexa
        "
    )
}

fn oktal_option() {
    print!(
        "
         7. Oktal   --> Decimal
         8. Oktal   --> Biner
         9. Oktal   --> Hexa
        "
    )
}

fn hexa_option() {
    print!(
        "
        10. Hexa    --> Decimal
        11. Hexa    --> Biner
        12. Hexa    --> Oktal
        "
    )
}
use std::any; // используем библиотеку для определения типов во время выполнения

// Функция принимает переменную и выводит информацию о ее типе
fn print_type<T>(var: T) {
    println!("Type for T: {}", any::type_name_of_val(&var));
}

fn main() {
    print_type(42); // выведет "Тип T: i32"
    print_type("Hello"); // выведет "Тип T: &str"
}

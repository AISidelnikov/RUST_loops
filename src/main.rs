fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        println!("counter = {}", counter);
        if counter == 10{
            break counter * 2;
        }
    };
    println!("result = {}", result);

    let mut number = 3;
    while number != 0{
        println!("{}!", number);
        number = number - 1;
    }
    println!("Поехали!");

    println!("WHILE");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5{
        println!("Значение равно {}", a[index]);
        index += 1;
    }

    println!("FOR");
    for element in a.iter(){
        println!("Значение равно {}", element);
    }

    for number in (1..4).rev(){
        println!("{}", number);
    }
}

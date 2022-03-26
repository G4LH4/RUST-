use std::io;

pub mod macros {
    #[macro_export]
    macro_rules! calculate_taxes {
        ($price:expr) => {
            let (iva, ip, ig) = (
                $price * 21.0 / 100.0,
                $price * 21.0 / 100.0,
                $price * 35.0/ 100.0
            );

            println!("");
            println!("Taxes: {}",iva + ip + ig);
            println!("");
            println!("The total price in ARGS is: ${:.2}", $price + iva + ip + ig);
            println!("");
        }
    }
}

fn main()
{
    println!("Enter The currency: ");

    let mut currency: String = String::new();
    let mut price_string: String = String::new();

    io::stdin()
        .read_line(&mut currency)
        .expect("Error");

    let price: f64 = match currency.as_str().trim()
    {
        "arg" => {
            println!("Enter the args: ");

            io::stdin()
                .read_line(&mut price_string)
                .expect("Not a valid nmb");

            price_string.trim().parse::<f64>().expect("Failed to parse price as f64")
        },
        "usd" => {
            println!("Enter the usd: ");

            io::stdin()
                .read_line(&mut price_string)
                .expect("Not a valid nmb");

            price_string.trim().parse::<f64>().expect("Failed to parse price as f64") * 121.83
        },
        _ => panic!("Unknown currency")
    };

    calculate_taxes!(price);

}



// fn calculate_taxes(price: f64)
// {
//     let (iva, ip, ig) = (
//         price * 21.0 / 100.0,
//         price * 21.0 / 100.0,
//         price * 35.0/ 100.0
//     );

//     println!("The total price in ARG pesos is: ${:.2}", price + iva + ip + ig);
// }
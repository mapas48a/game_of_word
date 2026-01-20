use std::{cmp::Ordering, io, ops::RangeInclusive};

use ran::ran_i64_range;


const WORDS:[&str;9] = [
    "Robot",
    "Lupa",
    "Perro",
    "Marco",
    "Grua",
    "Pistola",
    "Refresco",
    "Palomo",
    "Okey"
];


fn get_random_string() -> String{
    //el try_into() convierte el valor al tipo dado al declararlo ':' y devuelve un result equivalente a promise.
    let max_number_of_arrayray:u64 = WORDS.len()
    .try_into()
    .expect("no se pudo convertir el valor");

    let range_make:RangeInclusive<i64> = RangeInclusive::new(0, (max_number_of_arrayray - 1)
    .try_into()
    .expect("hubo un problema {self}"));
    
    //fix the issue with random
    let random_number:usize = ran_i64_range(range_make)
    .try_into()
    .expect("No se pudo resolver el cambio de datos");
    
    return WORDS[random_number].to_string()
}   


fn main() {
     // macro se ejecuta sola;
    let randomstring = get_random_string();

    if !randomstring.is_empty()  {
        //TODO: make the logic of the game
        let first_word = randomstring.chars().nth(0).unwrap();

        let last_word = randomstring.chars().nth(randomstring.len() - 1).unwrap();

        let len_of_word = randomstring.len().to_string();

        println!("Pista la Primera palabra es:{first_word} y la Ultima es:{last_word} y el largo de la palabra es {len_of_word}");
        
        loop {
            let mut input_user = String::new();

            io::stdin().read_line(&mut input_user).expect("Ocurrio un error al escribir");

            let input_user = input_user.trim().to_string();

            match input_user.cmp(&randomstring) {
                Ordering::Less => println!("te estas acercando... tu respuesta:{input_user}"),
                Ordering::Equal => {println!("Ganaste Felicidades!!! Acertaste {input_user}"); break;},
                Ordering::Greater => println!("te estas alejando... tu respuesta:{input_user}")
            }
        };
    }else {
        Err(randomstring).expect("A ocurrido un error")
    }

}

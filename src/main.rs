use std::io;
#[macro_use]
extern crate random_number;
struct Verbs {
    verb_simple_form: String,
    verb_simple_past: String,
    verb_past_participle: String,
    verb_spanish: String,
}

fn main() {
    let vector_verb: Vec<Verbs> = llenar_lista();
    println!("seleciona cuantos verbos Quiere aprender - maximo 30");
    let mut all_verbs = String::new();
    io::stdin().read_line(&mut all_verbs).expect("Fail");

    let all_verbs: i32 = all_verbs.trim().parse().expect("Fail to convert");
    let mut aux = 0;
    let mut vector_verbs_select_user: Vec<Verbs> = Vec::new();

    for elem in vector_verb.into_iter() {
        if aux < all_verbs {
            vector_verbs_select_user.push(elem);
        } else {
            break;
        }
        aux += 1;
    }

    verb_generator(&vector_verbs_select_user, all_verbs);
}

fn verb_generator(vector_verbs_user: &Vec<Verbs>, verbs: i32) {
    let mut i = 0;
    while i < 5 {
        println!("Verb in base form");
        let number_random: u32 = random!(0..=(verbs-1) as u32);
        println!("{}",number_random);
        let number_random_verbs = random!(1..=3);
        let verb_base_form = &vector_verbs_user[number_random as usize].verb_simple_form;
        println!("|{:-<25}|","-");
        println!("{:^25}", &verb_base_form);
        println!("|{:-<25}|","-");

     let  verb_select = match number_random_verbs {
            1 => {
                println!("ingrese el verbo en su forma participe");
                let aux = vector_verbs_user[number_random as usize].verb_past_participle.to_string();
                aux.to_string()
            }
            2 => {
                println!("ingrese el verbo en espanol");
                let aux = vector_verbs_user[number_random as usize].verb_spanish.to_string();
                aux.to_string()
            }
            3 => {
                println!("ingrese el verbo en past simple");
                let aux = vector_verbs_user[number_random as usize].verb_simple_past.to_string();
               aux.to_string()
            }
            _ => String::from("fail"),
        };
        check_answer(verb_select);
        i += 1;
    }
}

fn check_answer(select: String){
    let mut message = String::from("Respuesta correcta");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Fail");
    if select == response.trim() {
        println!("|{:-<25}|","-");
        println!("{:^28}",message);
        println!("|{:-<25}|","-");
    }else{
        message.push_str("Respuesta INCORRECTA");
        println!("|{:-<25}|","-");
        println!("{:^28}",String::from(message));
        println!("{:^25}",select);
        println!("|{:-<25}|","-");
    }
}

fn llenar_lista() -> Vec<Verbs> {
    let mut vec_verbs: Vec<Verbs> = Vec::new();
    let arreglo_verbs = [
        "beat",
        "beat",
        "beaten",
        "golpear",
        "bet",
        "bet",
        "bet",
        "apostar",
        "be",
        "was.were",
        "been",
        "ser.estar",
        "become",
        "became",
        "become",
        "llegar",
        "begin",
        "began",
        "begun",
        "empezar",
        "bend",
        "bent",
        "bent",
        "doblar",
        "bite",
        "bit",
        "bitten",
        "morder",
        "blow",
        "blew",
        "blown",
        "soplar",
        "break",
        "broke",
        "broken",
        "romper",
        "bring",
        "brought",
        "brought",
        "traer",
        "build",
        "built",
        "built",
        "construir",
        "buy",
        "bought",
        "bought",
        "comprar",
        "burst",
        "burst",
        "burst",
        "estallar",
        "catch",
        "caught",
        "caught",
        "atrapar",
        "choose",
        "chose",
        "chosen",
        "escoger",
        "come",
        "came",
        "come",
        "venir",
    ];
    let mut i = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    let mut d = 3;
    while i < 16 {
        vec_verbs.push(Verbs {
            verb_simple_form: String::from(arreglo_verbs[a]),
            verb_simple_past: String::from(arreglo_verbs[b]),
            verb_past_participle: String::from(arreglo_verbs[c]),
            verb_spanish: String::from(arreglo_verbs[d]),
        });
        a = d + 1;
        b = d + 2;
        c = d + 3;
        d = d + 4;
        i += 1;
    }

    vec_verbs
}

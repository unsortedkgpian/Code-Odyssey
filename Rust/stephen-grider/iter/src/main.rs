// Demo 
// LinkedList
use std::collections::LinkedList;

fn test_collect(elements: &[String]) -> LinkedList<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}


//iterators

fn print_elements(elements:&Vec<String>){
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements_iter(element:&Vec<String>){
    element.iter().for_each(|el| println!("{:#?}", el));
}

//Iterators adpator
fn print_elements_twice(elements:&Vec<String>){
    elements.iter().map(|el| format!("{} {}", el, el)).for_each(|el| println!("{:#?}", el));
}

fn print_element_vector_slice(elements:&[String]){
    elements.iter().for_each(|el| println!("{}",el));
}


// truncate() -> string metthod
fn shorten_strings(elements: &mut Vec<String>){
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn shorten_strings_slice(elements:&mut [String]){
    elements.iter_mut().for_each(|el| el.truncate(1));
}


fn to_uppercase(elements: &[String]) -> Vec<String>{
    elements.iter().map(|el| el.to_uppercase()).collect::<Vec<_>>()
}


fn move_elements(vec_a:Vec<String>, vec_b: &mut Vec<String>){
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}


fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.iter().map(|el| el.chars().map(|c| c.to_string()).collect()).collect()
}


fn find_color_or(elements: &[String], search:&str, fallback:&str) -> String {
    //elements.iter().map(|el| if el.contain(search) {return el.copy().to_string()});
    //fallback.copy().to_string()
    elements.iter().find(|el| el.contains(search)).map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    //println!("Hello, world!");
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("pink"),
        String::from("black"),
        String::from("white"),
        String::from("grey"),
    ];

    print_elements(&colors);
    print_elements_iter(&colors);
    print_elements_twice(&colors);
    print_element_vector_slice(&colors[2..4]);

    //shorten_strings(&mut colors);
    shorten_strings_slice(&mut colors[2..4]);
    print_elements_iter(&colors);

    let Colors = to_uppercase(&colors);
    print_elements_iter(&Colors);

    let Demo = to_uppercase(&colors);
    print_elements_iter(&Demo);
    println!("{:#?}",Demo);

    move_elements(Demo, &mut colors);
    println!("{:#?}", colors);


    let Exp = explode(&colors);
    println!("Explode {:#?}", Exp);
    let mut colors_iter = colors.iter();


    //println!("{:#?}",colors_iter);
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    //println!("{:#?}",colors_iter.next());
    let found_color = find_color_or(&colors, "re", "Orange")
}

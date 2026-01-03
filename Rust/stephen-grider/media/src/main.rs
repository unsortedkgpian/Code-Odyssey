#[derive(Debug)] 
enum Media{
    Book {title: String, author: String},
    Movie{title: String, director:String},
    Audiobook{title: String},
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String{
        //String::from("Media Description")
        
        //if let Media::Book {title, author} = self {
        //    format!("Book: {} {}", title, author)
        //}
        //else if let Media::Movie {title, director} = self {
        //    format!("Movie: {} {}", title, director)
        //}else if let Media::Audiobook {title} = self {
        //    format!("Audiobook: {}", title)
        //}else{
        //    String::from("Not a media Decscription")
        //}
        



        match self {
            Media::Book {title, author} => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie {title, director} => {
                format!("Movie: {} {}", title, director)
            }, 
            Media::Audiobook {title} => {
                format!("Audiobook {} ", title)
            },
            Media::Podcast(episode_number) => {
                format!("Podcast ep no : {}", episode_number)
            },
            Media::Placeholder => {
                String::from("Just a Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items:Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog {items: vec![]}
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self , index:usize) -> MightHaveAValue{
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        }else{
            MightHaveAValue::NoValueAvailable
        }
        //&self.items[index]


    }
}


enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

//fn print_book(book:Book)
//fn print_movie(movie:Movie)
//fn print_audiobook(audio:Audiobook)

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main(){
    println!("Hello, world!");

    let audiobook = Media::Audiobook{
        title: String::from("An Audiobook"),
    };
    
    let good_movie = Media::Movie{
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Money"),
        author: String::from("Adi Bytess"),
    };

    let podcast = Media::Podcast(23);
    let placeholder = Media::Placeholder;


    let mut catalog = Catalog::new();


    println!("{}",audiobook.description());
    println!("{}",good_movie.description());
    println!("{}",bad_book.description());
    println!("{}",podcast.description());
    println!("{}",placeholder.description());

    print_media(&audiobook);
    print_media(&good_movie);
    print_media(&bad_book);


    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
    println!("{:#?}",catalog.items.get(21));// -> None
    println!("{:#?}",catalog.items.get(1));//-> Some
                                        

    match catalog.items.get(23) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }

    //let item = catalog.get_by_index(2);
    //println!("{:#?}", item);

    //let undefineitem = catalog.get_by_index(24);
    //println!("{:#?}",undefineitem);
    
    match catalog.get_by_index(70) {
        MightHaveAValue::ThereIsAValue(value)=>{
            println!("Item: {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value out of index");
        }
    }

}

use std::ptr::null;

#[derive(Debug)]
enum Media{
    Book {Title:String, Auther:String},
    Movie {Title:String, Director:String},
    AudioBook {Title:String}
}// Book Movie and AudioBook will be of type Media

impl Media{
    fn description(&self) -> String{
    //     if let Media::Book { Title, Auther } = self{
    //         format!("book is {} by {}",Title,Auther)
    //     }else if let Media::Movie {Title,Director} = self {
    //         format!("movie is {} by {}",Title,Director)
    //     } else if let Media::AudioBook { Title } = self {
    //         format!("Audiobook is {}",Title)  
    //     } else{
    //         "error".to_string()
    //     }
    // }

        match self{
            Media::Book { Title, Auther } => {
                format!("book is {} by {}",Title,Auther)
            }
            Media::Movie { Title, Director } => {
                format!("movie is {} by {}",Title,Director)
            }
            Media::AudioBook { Title } => {
                format!("Audiobook is {}",Title)
            }
        }
    }
}

#[derive(Debug)]
struct Catalog{
    Items : Vec<Media>
}

impl Catalog{
    fn new() -> Self{
        Catalog {Items : vec![]}
    }
    fn add(&mut self, item: Media) {
        self.Items.push(item);
    }
}


fn main() {
    let mut catalog = Catalog::new();

    let audiobook = Media::AudioBook { 
        Title: ("Minecraft stories".to_string())
    };
    let movie = Media::Movie {
        Title: ("good movie".to_string()),
        Director: ("Good director".to_string()) 
        };

    let book = Media::Book {
        Title: "good book".to_string(),
        Auther: "good auther".to_string() 
        };

    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);


    println!("{}\n{}",catalog.Items[2].description(),catalog.Items[1].description());

    println!("{:#?}", catalog)
}

#[derive(Debug)]
enum Media{
    Book {title:String, auther:String},
    Movie {title:String, director:String},
    AudioBook {title:String},
    Podcast(u16),
    Placeholder
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
            Media::Book { title, auther } => {
                format!("book is {} by {}",title,auther)
            }
            Media::Movie { title, director } => {
                format!("movie is {} by {}",title,director)
            }
            Media::AudioBook { title } => {
                format!("Audiobook is {}",title)
            }
            Media::Podcast(ep_no) => {
                format!("ep no.{}",ep_no)
            }
            Media::Placeholder => {
                format!("N/A")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog{
    items : Vec<Media>
}

impl Catalog{
    fn new() -> Self{
        Catalog {items : vec![]}
    }
    fn add(&mut self, item: Media) {
        self.items.push(item);
    }
}


fn main() {
    let mut catalog = Catalog::new();

    let audiobook = Media::AudioBook { 
        title: ("Minecraft stories".to_string())
    };
    let movie = Media::Movie {
        title: ("good movie".to_string()),
        director: ("Good director".to_string()) 
        };

    let book = Media::Book {
        title: "good book".to_string(),
        auther: "good auther".to_string() 
        };


    let podcast = Media::Podcast(30);
    let placeholder = Media::Placeholder;


    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);


    println!("{}\n{}",catalog.items[2].description(),catalog.items[1].description());

    println!("{:#?}", catalog);
    println!("{:#?}", catalog.items.get(1));
}

mod content;
use content::catalog::Catalog;
use content::media::Media;


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

    println!("{:#?}", catalog.items[1]);
    
    let item = catalog.get_item(1);
    let item1 = catalog.get_item(40);
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap());//expecting there is a value here, if None the program panics
    println!("{:#?}",item1.unwrap_or(&placeholder));/*same as unwrap, if None gives another 
                                                    provided reference to a balue from Media*/
    println!("{:#?}",item1.expect("no value"));//like unwrap but sends the string if there is no value and crash
}

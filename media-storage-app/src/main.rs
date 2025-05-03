#[derive(Debug)]
enum Media{
    Book {Title:String, Auther:String},
    Movie {Title:String, Director:String},
    AudioBook {Title:String}
}// Book Movie and AudioBook will be of type Media

fn print_media(media:Media){
    println!("{:#?}",media);
}

fn main() {
    let audiobook = Media::AudioBook { 
        Title: ("Minecraft stories".to_string())
     };

     print_media(audiobook);
}

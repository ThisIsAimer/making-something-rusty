#[derive(Debug)]
pub enum Media{
    Book {title:String, auther:String},
    Movie {title:String, director:String},
    AudioBook {title:String},
    Podcast(u16),
    Placeholder
}// Book Movie and AudioBook will be of type Media

impl Media{
    pub fn description(&self) -> String{
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
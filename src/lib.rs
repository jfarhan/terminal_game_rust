pub mod graphics{


    pub const TOP_LEFT_CORNER:&'static str=" ";
    pub const TOP_RIGHT_CORNER:&'static str=" ";


    pub struct Screen{
        x_res:usize,
        y_res:usize,
        screen:Vec<Xcoords>,
    }
    #[derive(Debug)]
    struct Xcoords{
        x:Vec<char>
    }

    impl Screen{
        pub fn new_base_block(x_resn:usize,y_resn:usize)->Screen{
            let mut new_screen=Screen{
                x_res:x_resn,
                y_res:y_resn,
                screen:Vec::new()
            };


                for m in 0..y_resn{
                    new_screen.screen.push(Xcoords{x:Vec::new()});
                for _ in 0..x_resn{
                        new_screen.screen[m].x.push('=');
                }
            }

            new_screen

        }



        pub fn show_screen(&mut self){
            for row in self.screen.iter(){
                for element in row.x.iter(){
                    print!("{}",element);
                }
                println!("");
            }
        }
    }

}
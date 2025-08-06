pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

pub struct Button {
    width: u32,
    height: u32,
    text: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "DRAW Button(w:{}, h:{}, text:{})",
            self.width, self.height, self.text
        );
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "DRAW SelectBox(w:{}, h:{}, options:{:?})",
            self.width, self.height, self.options
        );
    }
}

pub fn test() {
    println!("[OOP] Start...");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                text: String::from("OK"),
            }),
        ],
    };

    screen.run();
    println!("[OOP] End...");
}

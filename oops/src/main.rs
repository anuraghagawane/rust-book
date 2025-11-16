use oops::{Button, Screen, SelectBox};
fn main() {
    let screen = Screen{
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: String::from("SB"),
            })
        ]
    };

    screen.run();
}

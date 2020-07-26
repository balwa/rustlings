// enums3.rs
// Address all the TODOs to make the tests pass!
use std::convert::TryFrom;

// Converting from larger value, we have to sanitize that the max value passed is not greater than u8
fn convert_func(v: i32) -> Option<u8> {
    if v > u8::MAX as i32 {
        None
    } else {
        Some(v as u8)
    }
}

enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit,
    Echo(String),
    Move(Point),
    ChangeColor(i32,i32,i32),
}

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message{
            Message::ChangeColor(r,g,b) => self.change_color((convert_func(r).unwrap(),convert_func(g).unwrap(),convert_func(b).unwrap())),
            Message::Echo(s) => self.echo(s),
            Message::Quit => self.quit(),
            Message::Move(Point) => self.move_position(Point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State{
            quit: false,
            position: Point{ x: 0, y: 0 },
            color: (0, 0, 0)
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point{ x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}

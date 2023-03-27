struct Position {
    x: u16,
    y: u16,
}

struct Size {
    height: u16,
    width: u16,
}

pub struct Rect {
    position: Position,
    size: Size,
}

// ╭ (U+256D) - Top left corner
// ╮ (U+256E) - Top right corner
// ╰ (U+2570) - Bottom left corner
// ╯ (U+256F) - Bottom right corner
// ─ (U+2500) - Horizontal line
// │ (U+2502) - Vertical line


enum BorderChars {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    HorizontalLine,
    VerticalLine,
}

impl ToString for BorderChars {
    fn to_string(&self) -> String {
        match self {
            BorderChars::TopLeftCorner => "┌".to_string(),
            BorderChars::TopRightCorner => "┐".to_string(),
            BorderChars::BottomLeftCorner => "└".to_string(),
            BorderChars::BottomRightCorner => "┘".to_string(),
            BorderChars::HorizontalLine => "─".to_string(),
            BorderChars::VerticalLine => "│".to_string(),
        }
    }
}

impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        let position = Position { x, y };
        let size = Size {
            width: w,
            height: h,
        };
        Rect { position, size }
    }
    pub fn draw(&self) {
        // horizontal
        for i in self.position.x..self.size.width {
            print!("{}", termion::cursor::Goto(i, self.position.y));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        for i in self.position.x..self.size.width {
            print!("{}", termion::cursor::Goto(i, self.size.height));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        // vertical
        for i in self.position.y..self.size.height {
            print!("{}", termion::cursor::Goto(self.position.x, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        for i in self.position.y..self.size.height {
            print!("{}", termion::cursor::Goto(self.size.width, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        print!("{}", termion::cursor::Goto(self.position.x, self.position.y));
        print!("{:}", BorderChars::TopLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(self.size.width, self.position.y));
        print!("{:}", BorderChars::TopRightCorner.to_string());
        print!("{}", termion::cursor::Goto(self.position.x, self.size.height));
        print!("{:}", BorderChars::BottomLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(self.size.width, self.size.height));
        print!("{:}", BorderChars::BottomRightCorner.to_string());

    }
}

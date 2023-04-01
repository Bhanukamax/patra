#[derive(Clone)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn add(&self, pos: Self) -> Self {
        let x = self.x + pos.x;
        let y = self.y + pos.y;
        Position { x, y }
    }
}

struct Size {
    height: u16,
    width: u16,
}

pub struct Rect {
    position: Position,
    size: Size,
    cursor: Position,
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
    pub fn add_line(&mut self, text: &str) {
        print!("{}", termion::cursor::Goto(self.cursor.x, self.cursor.y));
        print!("{:}", text);
        self.cursor.y += 1;
    }

    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        let position = Position { x, y };
        let cursor = Position {
            x: x.saturating_add_signed(1),
            y: y.saturating_add(1),
        };
        let size = Size {
            width: w,
            height: h,
        };
        Rect {
            position,
            size,
            cursor,
        }
    }
    pub fn draw(&self) {
        // horizontal

        let Size { width, height } = self.size;
        let end_pos = self.position.clone().add(Position {
            x: width,
            y: height,
        });

        for i in self.position.x..end_pos.x {
            print!("{}", termion::cursor::Goto(i, self.position.y));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        for i in self.position.x..end_pos.x {
            print!("{}", termion::cursor::Goto(i, self.size.height));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        // vertical
        for i in self.position.y..self.size.height {
            print!("{}", termion::cursor::Goto(self.position.x, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        for i in self.position.y..self.size.height {
            print!("{}", termion::cursor::Goto(end_pos.x, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        print!(
            "{}",
            termion::cursor::Goto(self.position.x, self.position.y)
        );
        print!("{:}", BorderChars::TopLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(end_pos.x, self.position.y));
        print!("{:}", BorderChars::TopRightCorner.to_string());
        print!(
            "{}",
            termion::cursor::Goto(self.position.x, self.size.height)
        );
        print!("{:}", BorderChars::BottomLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(end_pos.x, self.size.height));
        print!("{:}", BorderChars::BottomRightCorner.to_string());
    }
}

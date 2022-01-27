use crate::code_point::CodePoint;

pub(crate) trait LineBreaker {
    fn check_if_new_line(&mut self, code_point: &CodePoint) -> bool;
}

enum History {
    Lf,
    Cr,
    NewLine,
    Other,
}

pub(crate) struct LinuxOrWindowsLineBreaker {
    history: History,
}

impl LinuxOrWindowsLineBreaker {
    pub(crate) fn new() -> LinuxOrWindowsLineBreaker {
        LinuxOrWindowsLineBreaker { history: History::NewLine }
    }
}

mod code_points {
    use crate::code_point::CodePoint;

    pub(crate) const LF: CodePoint = CodePoint { char: 10 };
    pub(crate) const CR: CodePoint = CodePoint { char: 13 };
}

impl LineBreaker for LinuxOrWindowsLineBreaker {
    fn check_if_new_line(&mut self, code_point: &CodePoint) -> bool {
        let (history_new, starts_new_line) =
            if *code_point == code_points::LF {
                match self.history {
                    History::Lf => { (History::Lf, true) }
                    History::Cr => { (History::NewLine, false) }
                    History::NewLine => { (History::Lf, true) }
                    History::Other => { (History::Lf, false) }
                }
            } else if *code_point == code_points::CR {
                match self.history {
                    History::Lf => { (History::NewLine, false) }
                    History::Cr => { (History::Cr, true) }
                    History::NewLine => { (History::Cr, true) }
                    History::Other => { (History::Cr, false) }
                }
            } else {
                match self.history {
                    History::Lf => { (History::Other, true) }
                    History::Cr => { (History::Other, true) }
                    History::NewLine => { (History::Other, true) }
                    History::Other => { (History::Other, false) }
                }
            };
        self.history = history_new;
        starts_new_line
    }
}


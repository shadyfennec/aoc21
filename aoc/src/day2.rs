use crate::AocDay;

pub struct Day2;

impl Default for Day2 {
    fn default() -> Self {
        Self {}
    }
}

enum CommandKind {
    Up,
    Down,
    Forward,
}

struct Command {
    kind: CommandKind,
    amount: isize,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let mut elements = s.split_ascii_whitespace();
        let kind = elements.next().unwrap();
        let amount = elements.next().unwrap();

        let kind = match kind {
            "forward" => CommandKind::Forward,
            "up" => CommandKind::Up,
            "down" => CommandKind::Down,
            _ => unreachable!(),
        };

        Command {
            kind,
            amount: amount.parse().unwrap(),
        }
    }
}

impl AocDay for Day2 {
    fn title(&self) -> String {
        "Dive!".to_owned()
    }

    fn inputs(&self) -> &[&'static str] {
        &["inputs/day2/small.txt", "inputs/day2/real.txt"]
    }

    fn part_1(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        _debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let (depth, position) = input.into_iter().map(|s| s.into()).fold(
            (0, 0),
            |(mut depth, mut position), c: Command| {
                match c.kind {
                    CommandKind::Up => depth -= c.amount,
                    CommandKind::Down => depth += c.amount,
                    CommandKind::Forward => position += c.amount,
                };
                (depth, position)
            },
        );

        self.println(format!("{}", depth * position), &output);

        Ok(())
    }

    fn part_2(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        _debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let (depth, position, _) = input.into_iter().map(|s| s.into()).fold(
            (0, 0, 0),
            |(mut depth, mut position, mut aim), c: Command| {
                match c.kind {
                    CommandKind::Up => aim -= c.amount,
                    CommandKind::Down => aim += c.amount,
                    CommandKind::Forward => {
                        position += c.amount;
                        depth += c.amount * aim;
                    }
                };
                (depth, position, aim)
            },
        );

        self.println(format!("{}", depth * position), &output);

        Ok(())
    }
}

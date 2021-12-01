use crate::AocDay;

use itertools::Itertools;

pub struct Day1;

impl Default for Day1 {
    fn default() -> Self {
        Self {}
    }
}

impl AocDay for Day1 {
    fn title(&self) -> String {
        "Sonar Sweep".to_owned()
    }

    fn inputs(&self) -> &[&'static str] {
        &["inputs/day1/small.txt", "inputs/day1/real.txt"]
    }

    fn part_1(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        _debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let result = input
            .into_iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .fold((None, 0), |(a, mut n), b| {
                if let Some(depth) = a {
                    if b > depth {
                        n += 1;
                    }
                };

                (Some(b), n)
            })
            .1;

        self.println(format!("{}", result), &output);

        Ok(())
    }

    fn part_2(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        _debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let result = input
            .into_iter()
            .filter_map(|s| s.parse::<usize>().ok())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .fold((None, 0), |(a, mut n), b| {
                if let Some(depth) = a {
                    if b > depth {
                        n += 1;
                    }
                };

                (Some(b), n)
            })
            .1;

        self.println(format!("{}", result), &output);
        Ok(())
    }
}

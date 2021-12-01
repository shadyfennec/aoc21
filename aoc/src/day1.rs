use crate::AocDay;

pub struct Day1;

impl Default for Day1 {
    fn default() -> Self {
        Self {}
    }
}

impl AocDay for Day1 {
    fn title(&self) -> String {
        "The first day of Santa".to_owned()
    }

    fn description(&self) -> String {
        "a".to_owned()
    }

    fn inputs(&self) -> &[&'static str] {
        &["inputs/day1/small.txt", "inputs/day1/real.txt"]
    }

    fn run(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        debug.send(String::from("debugging!!!!!!!!!!!"))?;

        std::thread::sleep(std::time::Duration::from_millis(2000));

        let _ = input
            .into_iter()
            .try_for_each(|_| Result::<(), color_eyre::Report>::Ok(()))?;

        output.send(String::from("all good :)"))?;

        Ok(())
    }
}

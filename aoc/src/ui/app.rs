use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;

use tui::style::{Color, Modifier, Style};

use crate::ui::ThreadPool;
use crate::AocDay;

enum State {
    SelectingDay,
    SelectingInput,
}

type ArcDay = Arc<Box<dyn AocDay + Sync + Send>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum JobStatus {
    Ready,
    Waiting,
    Running(usize),
    Finished(Duration),
    Error,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Ready => write!(f, "Ready"),
            JobStatus::Waiting => write!(f, "Waiting"),
            JobStatus::Running(id) => write!(f, "Running ({})", id),
            JobStatus::Error => write!(f, "Error"),
            JobStatus::Finished(_) => write!(f, "Finished"),
        }
    }
}

impl JobStatus {
    pub fn style(&self) -> Style {
        match self {
            JobStatus::Ready => Style::default(),
            JobStatus::Waiting => Style::default().fg(Color::Yellow),
            JobStatus::Running(_) => Style::default().fg(Color::Blue),
            JobStatus::Error => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            JobStatus::Finished(_) => Style::default().fg(Color::Green),
        }
    }
}

pub struct OutputCommunication {
    output: (Sender<String>, Receiver<String>),
    debug: (Sender<String>, Receiver<String>),
}

impl OutputCommunication {
    pub fn new() -> Self {
        Self {
            output: mpsc::channel(),
            debug: mpsc::channel(),
        }
    }

    pub fn senders(&self) -> (Sender<String>, Sender<String>) {
        (self.output.0.clone(), self.debug.0.clone())
    }
}

pub struct Instance {
    pub(crate) input: &'static str,
    pub(crate) job_id: Option<usize>,
    pub(crate) status: JobStatus,
    pub(crate) communication: OutputCommunication,
}

impl Instance {
    pub fn duration(&self) -> Option<String> {
        if let JobStatus::Finished(d) = &self.status {
            Some(format!(
                "{:02}:{:02}.{:03}",
                d.as_secs() / 60,
                d.as_secs() % 60,
                d.as_millis() % 1000
            ))
        } else {
            None
        }
    }
}

pub struct Day {
    pub(crate) number: usize,
    pub(crate) day: Option<ArcDay>,
    pub(crate) instances: Vec<Instance>,
}

impl From<crate::Day> for Day {
    fn from(d: crate::Day) -> Self {
        let (number, b) = d;

        let instances = if let Some(b) = &b {
            b.inputs()
                .iter()
                .map(|i| Instance {
                    input: i,
                    job_id: None,
                    status: JobStatus::Ready,
                    communication: OutputCommunication::new(),
                })
                .collect()
        } else {
            Vec::new()
        };

        Day {
            number,
            day: b.map(Arc::from),
            instances,
        }
    }
}

impl Day {
    pub fn status(&self) -> JobStatus {
        self.instances.iter().map(|i| i.status).max().unwrap()
    }
}

pub struct App {
    pub(crate) days: Vec<Day>,
    pub(crate) day_highlight: Option<usize>,
    pub(crate) input_highlight: Option<usize>,
    pool: ThreadPool<4>,
    state: State,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            days: crate::days().into_iter().map(Into::into).collect(),
            day_highlight: Some(0),
            input_highlight: None,
            pool: ThreadPool::new(),
            state: State::SelectingDay,
            should_quit: false,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn day_selection(&mut self) {
        self.state = State::SelectingDay;
        self.input_highlight = None;
    }

    fn input_selection(&mut self) {
        if let Some(n) = self.day_highlight {
            if self.days.get(n).unwrap().day.is_some() {
                self.state = State::SelectingInput;
                self.input_highlight = Some(0);
            }
        }
    }

    pub fn handle_key(&mut self, letter: char) {
        match letter {
            'q' | 'Q' => match self.state {
                State::SelectingDay => self.should_quit = true,
                State::SelectingInput => self.day_selection(),
            },
            'r' => self.input_selection(),
            'R' => self.run_all(),
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        match self.state {
            State::SelectingDay => {
                if let Some(h) = self.day_highlight {
                    if h == 0 {
                        self.day_highlight = Some(24)
                    } else {
                        self.day_highlight = Some(h - 1)
                    }
                } else {
                    self.day_highlight = Some(0)
                }
                self.input_highlight = None;
            }
            State::SelectingInput => {
                let day = self.days.get(self.day_highlight.unwrap()).unwrap();
                let input_size = day.instances.len();
                if let Some(h) = self.input_highlight {
                    if h == 0 {
                        self.input_highlight = Some(input_size - 1)
                    } else {
                        self.input_highlight = Some(h - 1)
                    }
                } else {
                    self.input_highlight = Some(0)
                }
            }
        }
    }

    pub fn on_down(&mut self) {
        match self.state {
            State::SelectingDay => {
                if let Some(h) = self.day_highlight {
                    if h == 24 {
                        self.day_highlight = Some(0)
                    } else {
                        self.day_highlight = Some(h + 1)
                    }
                } else {
                    self.day_highlight = Some(0)
                }
                self.input_highlight = None;
            }
            State::SelectingInput => {
                let day = self.days.get(self.day_highlight.unwrap()).unwrap();
                let input_size = day.instances.len();
                if let Some(h) = self.input_highlight {
                    if h == input_size - 1 {
                        self.input_highlight = Some(0)
                    } else {
                        self.input_highlight = Some(h + 1)
                    }
                } else {
                    self.input_highlight = Some(0)
                }
            }
        }
    }

    pub fn on_tick(&mut self) {
        let report = self.pool.update();

        self.days.iter_mut().for_each(|day| {
            day.instances.iter_mut().for_each(|i| {
                if let Some(id) = i.job_id {
                    if let Some((_, worker_id)) =
                        report.started_jobs.iter().find(|started| started.0 == id)
                    {
                        i.status = JobStatus::Running(*worker_id)
                    } else if let Some(result) = report
                        .finished_jobs
                        .iter()
                        .find(|finished| finished.0 == id)
                    {
                        i.status = match result.1 {
                            Ok(d) => JobStatus::Finished(d),
                            Err(_) => JobStatus::Error,
                        }
                    }
                }
            })
        });
    }

    fn run_input(&mut self) {
        if let Some(i) = self.day_highlight {
            let day = self.days.get_mut(i).unwrap();

            if let Some(i) = self.input_highlight {
                let instance = day.instances.get_mut(i).unwrap();
                let day = day.day.as_ref().unwrap().clone();

                let (output, debug) = instance.communication.senders();
                let input = instance.input.to_string();

                let (job_id, worker_id) = self
                    .pool
                    .register(move || day.run_timed(input, output, debug));

                instance.job_id = Some(job_id);
                instance.status = worker_id
                    .map(JobStatus::Running)
                    .unwrap_or(JobStatus::Waiting);
            }
        }
    }

    fn run_all(&mut self) {
        self.days.iter_mut().for_each(|d| {
            if let Some(a) = d.day.as_ref() {
                d.instances.iter_mut().for_each(|i| {
                    let (output, debug) = i.communication.senders();
                    let input = i.input.to_string();

                    let a = a.clone();

                    let (job_id, worker_id) = self
                        .pool
                        .register(move || a.run_timed(input, output, debug));

                    i.job_id = Some(job_id);
                    i.status = worker_id
                        .map(JobStatus::Running)
                        .unwrap_or(JobStatus::Waiting);
                })
            }
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

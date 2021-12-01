use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

type JobResult = Result<Duration, color_eyre::Report>;

pub struct Worker {
    id: usize,
    job: Option<usize>,
    transmitter: Sender<Job>,
    receiver: Receiver<JobResult>,
    quit_tx: Sender<()>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        let (data_tx, data_rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let (status_tx, status_rx) = mpsc::channel();
        let (quit_tx, quit_rx) = mpsc::channel();

        let handle = thread::spawn(move || loop {
            match quit_rx.try_recv() {
                Ok(_) => break,
                Err(mpsc::TryRecvError::Disconnected) => break,
                Err(_) => {}
            };

            match data_rx.recv_timeout(Duration::from_millis(16)) {
                Ok(f) => status_tx.send((f.closure)()).unwrap(),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => panic!("Disconnected"),
            }
        });

        Self {
            id,
            job: None,
            transmitter: data_tx,
            receiver: status_rx,
            quit_tx,
            handle: Some(handle),
        }
    }

    pub fn is_running(&self) -> bool {
        self.job.is_some()
    }

    pub fn run_job(&mut self, job: Job) -> Result<usize, ()> {
        if self.is_running() {
            Err(())
        } else {
            let id = job.id;
            self.job = Some(id);
            self.transmitter.send(job).unwrap();
            Ok(id)
        }
    }

    pub fn update(&mut self) -> Option<(usize, JobResult)> {
        match self.receiver.try_recv() {
            Ok(result) => self.job.take().map(|i| (i, result)),
            Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
            Err(mpsc::TryRecvError::Empty) => None,
        }
    }
}

type BoxedFn = Box<dyn FnOnce() -> JobResult + Send + 'static>;

static JOB_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Job {
    id: usize,
    closure: BoxedFn,
}

impl Job {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> JobResult + Send + 'static,
    {
        Job {
            id: JOB_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            closure: Box::new(f),
        }
    }
}

pub struct UpdateReport {
    pub finished_jobs: Vec<(usize, JobResult)>,
    pub started_jobs: Vec<(usize, usize)>,
}

pub struct ThreadPool<const THREADS: usize> {
    workers: Vec<Worker>,
    jobs: VecDeque<Job>,
}

impl<const THREADS: usize> ThreadPool<THREADS> {
    pub fn new() -> Self {
        let mut workers = Vec::with_capacity(THREADS);
        workers.extend((0..THREADS).map(Worker::new));
        Self {
            workers,
            jobs: VecDeque::new(),
        }
    }

    fn next_available(&mut self) -> Option<&mut Worker> {
        self.workers.iter_mut().find(|w| w.is_running())
    }

    pub fn register<F>(&mut self, f: F) -> (usize, Option<usize>)
    where
        F: FnOnce() -> Result<Duration, color_eyre::Report> + Send + 'static,
    {
        let job = Job::new(f);
        let job_id = job.id;
        if let Some(w) = self.next_available() {
            let worker_id = w.id;
            w.run_job(job).unwrap();

            (job_id, Some(worker_id))
        } else {
            self.jobs.push_back(job);
            (job_id, None)
        }
    }

    pub fn update(&mut self) -> UpdateReport {
        let finished_jobs = self.workers.iter_mut().filter_map(|w| w.update()).collect();

        let started_jobs = self
            .workers
            .iter_mut()
            .filter(|w| !w.is_running())
            .filter_map(|w| {
                self.jobs.pop_front().map(|j| {
                    let job_id = j.id;
                    w.run_job(j).unwrap();
                    (job_id, w.id)
                })
            })
            .collect();

        UpdateReport {
            finished_jobs,
            started_jobs,
        }
    }
}

impl<const THREADS: usize> Drop for ThreadPool<THREADS> {
    fn drop(&mut self) {
        self.workers.iter_mut().for_each(|w| {
            w.quit_tx.send(()).unwrap();
            w.handle.take().unwrap().join().unwrap();
        })
    }
}

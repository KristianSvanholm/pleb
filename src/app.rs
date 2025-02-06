use rand::rng;
use std::collections::HashMap;
use std::error;

use crate::benchmark::{self, Export, Task};
use crate::event::Event;
use crate::Mode;
use rand::seq::SliceRandom;
use std::{thread, time};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Display {
    Percent,
    Fraction,
}

// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// Application
#[derive(Debug)]
pub struct App {
    pub sender: mpsc::UnboundedSender<Event>,
    pub running: bool,

    pub mode: Mode,
    pub display_mode: Display,
    pub runs: u64,
    pub ordered: bool,
    pub cooldown: u64,
    pub curr_task: usize,
    pub status: HashMap<(String, String), u64>, // Track completion count
    pub status_text: String,
    pub tasks: Vec<Task>,                 // Todo task list
    pub task_count: HashMap<String, u64>, // Count of unique languages for each task
    pub lang_count: HashMap<String, u64>, // Count of unique tasks for each language

    pub results: Vec<Export>, // Results
}

impl App {
    pub fn new(tasks: Vec<Task>, mode: crate::Mode, sender: mpsc::UnboundedSender<Event>) -> Self {
        let mut _runs = 1;
        let mut _ordered = true;
        let mut _cooldown = 0;
        match mode {
            Mode::Run { runs, ordered, cooldown } => {
                _runs = runs;
                _ordered = ordered;
                _cooldown = cooldown;
            }
            Mode::Compile => (),
        };

        // Sum up task counts
        let task_count = tasks.iter().fold(HashMap::new(), |mut map, t| {
            *map.entry(t.name.clone()).or_insert(0) += 1;
            map
        });

        // Sum up language counts
        let lang_count = tasks.iter().fold(HashMap::new(), |mut map, l| {
            *map.entry(l.language.clone()).or_insert(0) += 1;
            map
        });

        // Initialiez status map
        let status = tasks.iter().fold(HashMap::new(), |mut map, t| {
            map.entry((t.language.clone(), t.name.clone())).or_insert(0);
            map
        });

        // n runs of each task
        let mut t = vec![];
        for ut in tasks {
            for _ in 0.._runs {
                t.push(ut.clone());
            }
        }
        let mut tasks = t;
        if !_ordered {
            tasks.shuffle(&mut rng());
        }

        Self {
            mode,
            display_mode: Display::Percent,
            sender,
            running: true,
            status,
            status_text: String::from(""),

            curr_task: 0,
            runs: _runs,
            cooldown: _cooldown,
            ordered: _ordered,
            tasks,
            task_count,
            lang_count,
            results: vec![],
        }
    }

    pub fn done(&mut self, data: benchmark::Export) {
        self.results.push(data.clone());
        self.next((data.language, data.task));
    }

    pub fn next(&mut self, data: (String, String)) {
        self.curr_task += 1;
        self.update(data);

        self.iterate(); // Run next benchmark / compile
    }

    pub fn iterate(&mut self) {
        if self.curr_task == self.tasks.len() {
            self.status_text = String::from("Done!");
            self.results
                .sort_unstable_by_key(|item| (item.language.to_owned(), item.task.to_owned()));
            return;
        }

        // Create new tokio thread, pass in event writer, run next benchmark
        let _sender = self.sender.clone();
        let _task = self.tasks[self.curr_task].clone();

        // Status run message
        let run_status = format!(
            "Running {} {} - {}/{}",
            _task.language,
            _task.name,
            self.status.get(&(_task.language.clone(), _task.name.clone())).expect("") + 1,
            self.runs
        );

        let run_msg = matches!(self.mode ,Mode::Run { .. });

        let first = self.curr_task == 0;
        let c = self.cooldown.clone();

        let mode = self.mode.clone();
        tokio::spawn(async move {
            if !first && run_msg {
                // Cooldown
                let _ = _sender.send(Event::Status(format!("Cooling down for {} second(s)", c)));
                thread::sleep(time::Duration::from_secs(c));
            }

           
            if run_msg {
                let _ = _sender.send(Event::Status(run_status));
            }

            match mode {
                Mode::Run { .. } => {
                    let export = benchmark::run(_task);
                    _sender.send(Event::TaskDone(export))
                }
                Mode::Compile => {
                    let out = benchmark::compile(&_task);
                    let _ = _sender.send(Event::Status(out));
                    _sender.send(Event::CompileDone(_task))
                }
            }
        });
    }

    fn update(&mut self, data: (String, String)) {
        // Update status data
        self.status.entry(data).and_modify(|f| *f += 1);
    }

    pub fn status(&mut self, msg: String) {
        self.status_text = msg;
    }

    pub fn toggle_display_mode(&mut self) {
        self.display_mode = match self.display_mode {
            Display::Percent => Display::Fraction,
            Display::Fraction => Display::Percent,
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false
    }
}

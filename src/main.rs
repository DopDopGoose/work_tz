use std::fmt::{Display};
use crossterm::{cursor, execute, terminal::{Clear, ClearType}, ExecutableCommand};
use lazy_static::lazy_static;
use std::time::Duration;
use std::thread::sleep;
use std::io::{stdout, Write};
use rand;
use rand::Rng;

static ONE_DAY_HOURS: u32 = 12;

lazy_static!(
    static ref SLOW_WRITE_TIME: Duration = Duration::from_millis(100);
);

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum State {
    Working,
    Skipping,
}

impl State {
    pub fn to_string(&self) -> String {
        match self {
            State::Working => "Working".to_string(),
            State::Skipping => "Skipping".to_string()
        }
    }
}

pub struct Worker {
    state: State,
    name: String,
    working_time: u32,
    last_working_day: Option<u32>,
    skips_count: u32,
    on_work_count: u32
}

impl Worker {
    pub fn with_name(name: &str) -> Self {
        Worker { state: State::Skipping,
            name: name.to_string(),
            working_time: 0,
            last_working_day: None,
            skips_count: 0,
            on_work_count: 0}
    }

    pub fn go_to_work_or_not_go_to_work(&mut self) -> &State {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1) {
            0 => { self.state = State::Skipping; self.skips_count += 1; &self.state },
            1 => { self.state = State::Working; self.on_work_count +=1; self.last_working_day = Some(self.on_work_count); &self.state }
            _ => { &self.state }
        }
    }

    pub fn add_working_time(&mut self, time: u32) {
        self.working_time += time;
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} | {} | {} | skips: {} | worked_days: {} | last_worked_day: {}",
               self.name,
               self.state.to_string(),
               self.working_time,
               self.skips_count,
               self.on_work_count,
               { match self.last_working_day { Some(v) => v.to_string(), None => "None".to_string() } }
        );
    }

    pub fn add_working_time_if_on_work(&mut self, time: u32) {
        if self.state == State::Skipping { return; }
        self.working_time += time;
    }
}

pub fn write_slowly(text: &str) {
    for char in text.chars() {
        print!("{}", char);
        stdout().flush();
        sleep(SLOW_WRITE_TIME.clone());
    }
    println!();
}


fn main() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    let mut WORKERS: Vec<Worker> = vec![
        Worker::with_name("Alehandro"),
        Worker::with_name("Jane"),
        Worker::with_name("Pipidastr"),
        Worker::with_name("Sergey"),
        Worker::with_name("Kringe"),
        Worker::with_name("Fedor"),
    ];
    let mut working_day: u32 = 0;

    write_slowly(&format!("Набрано {} работников, кто не работает, тот не ест...", WORKERS.len()), );
    loop {
        working_day += 1;
        write_slowly(&format!("День {}\nРабочий день начинается", working_day));

        for mut worker in &mut WORKERS {
            worker.go_to_work_or_not_go_to_work();
        }

        for i in 0..ONE_DAY_HOURS {
            let to_write: String = format!("День {}\nПрошло времени: {}:00:00\n====Статус работников=====\n{}", working_day, i, WORKERS.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n"));
            println!("{}", to_write);

            stdout().flush();
            sleep(Duration::from_secs(1));
            execute!(stdout(), Clear(ClearType::All)).unwrap();
            stdout().flush().unwrap();
            execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

            for worker in &mut WORKERS { worker.add_working_time_if_on_work(1); }
        }
        write_slowly("Рабочий день окончен... Все идут домой");
    }
}

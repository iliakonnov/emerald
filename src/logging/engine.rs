pub(crate) enum Log {
    Info(String),
    Warning(String),
    Error(String),
}

pub struct LoggingEngine {
    logs: Vec<Log>,
}
impl LoggingEngine {
    pub(crate) fn new() -> Self {
        LoggingEngine { logs: Vec::new() }
    }

    pub fn update(&mut self) {
        for log in &self.logs {
            #[cfg(not(target_arch = "wasm32"))]
            {
                match log {
                    Log::Info(msg) => println!("{}", msg),
                    Log::Warning(msg) => println!("{}", msg),
                    Log::Error(msg) => println!("{}", msg),
                }
            }
        }

        self.logs = Vec::with_capacity(self.logs.len());
    }

    fn log(&mut self, log: Log) {
        self.logs.push(log);
    }

    pub fn info<T: Into<String>>(&mut self, msg: T) {
        let log = Log::Info(msg.into());

        self.log(log);
    }

    pub fn warning<T: Into<String>>(&mut self, msg: T) {
        let log = Log::Warning(msg.into());

        self.log(log);
    }

    pub fn error<T: Into<String>>(&mut self, msg: T) {
        let log = Log::Error(msg.into());

        self.log(log);
    }
}

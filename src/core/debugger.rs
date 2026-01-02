use crate::errors::Result;

#[allow(dead_code)]
pub trait DebuggerBackend {
    /// Attach to a running process
    fn attach(&mut self, pid: u32) -> Result<()>;

    /// Spawn a new process and attach to it
    fn spawn(&mut self, path: &str) -> Result<()>;

    /// Set a breakpoint at a specific address
    fn set_breakpoint(&mut self, address: u64) -> Result<()>;

    /// List all breakpoints
    fn list_breakpoints(&self) -> Vec<u64>;

    /// Single step the process
    fn step(&mut self) -> Result<()>;

    /// Continue execution
    fn continue_execution(&mut self) -> Result<()>;

    /// Read registers
    fn read_registers(&self) -> Result<Registers>;

    /// Read memory
    fn read_memory(&self, address: u64, size: usize) -> Result<Vec<u8>>;
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Registers {
    pub rip: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    // Add more registers as needed for specific architectures
}

#[allow(dead_code)]
pub struct Debugger {
    backend: Box<dyn DebuggerBackend>,
}

impl Debugger {
    pub fn new(backend: Box<dyn DebuggerBackend>) -> Self {
        Self { backend }
    }

    // High level methods can be added here
}

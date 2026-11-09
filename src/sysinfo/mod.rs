mod cpu;
mod gpu;
mod system;

pub use cpu::cpu_g;
pub use gpu::get_gpu_name;
pub use system::{host, os, ram_gib, user};

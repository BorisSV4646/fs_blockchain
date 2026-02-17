mod consensus;
mod mining;
mod network;
mod node;
mod storage;
mod sync;

fn main() {
    if let Err(err) = node::run() {
        eprintln!("Ошибка запуска ноды: {err}");
    }
}

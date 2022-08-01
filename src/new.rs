use crate::config::New;
use crate::workspace::Workspace;

pub fn run(cfg: &New) {
    let mut ws = Workspace::new(&cfg.project_name, &cfg.directory_name);
    ws.fill_from_user_input();
    ws.write_to_disk();
}

use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

pub struct Player {
    instance: Child,
}

impl Player {
    pub fn new() -> Self {
        Self {
            instance: Self::spawn(),
        }
    }

    pub fn load(&mut self, video: &str) {
        self.check();

        Command::new("cmd")
            .arg("/C")
            .arg(format!("echo loadfile {} > \\\\.\\pipe\\mpv-pipe", video))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        Command::new("cmd")
            .arg("/C")
            .arg("echo set pause no > \\\\.\\pipe\\mpv-pipe")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn stop(&mut self) {
        self.check();

        Command::new("cmd")
            .arg("/C")
            .arg("echo set pause yes > \\\\.\\pipe\\mpv-pipe")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    pub fn kill(&mut self) {
        let _ = self.instance.kill();
    }

    fn check(&mut self) {
        if let Ok(op) = self.instance.try_wait() {
            if op.is_none() {
                return;
            }
        }

        self.instance = Self::spawn();

        let pipe = Path::new("\\\\.\\pipe\\mpv-pipe");

        while !pipe.exists() {}
    }

    fn spawn() -> Child {
        Command::new("mpv.exe")
            .args(&[
                "--http-header-fields=Referer: https://kwik.cx",
                "--input-ipc-server=\\\\.\\pipe\\mpv-pipe",
                "--volume=40",
            ])
            .spawn()
            .unwrap()
    }
}

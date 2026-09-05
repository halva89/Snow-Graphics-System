use crate::scene_parser::mesh_loader::{load_obj, LoadedMesh};
use std::sync::mpsc::{channel, Receiver, Sender};

type JobId = u64;

enum LoadTask {
    Obj { id: JobId, path: String },
    Shutdown,
}

pub struct LoadResult {
    pub id: JobId,
    pub loaded: Option<LoadedMesh>,
}

pub struct AssetLoader {
    tx: Sender<LoadTask>,
    rx: Receiver<LoadResult>,
    next_id: JobId,
}

impl AssetLoader {
    pub fn spawn() -> Self {
        let (task_tx, task_rx) = channel::<LoadTask>();
        let (res_tx, res_rx) = channel::<LoadResult>();
        std::thread::spawn(move || {
            for task in task_rx {
                match task {
                    LoadTask::Obj { id, path } => {
                        let loaded = load_obj(&path);
                        if res_tx.send(LoadResult { id, loaded }).is_err() {
                            break;
                        }
                    }
                    LoadTask::Shutdown => break,
                }
            }
        });
        Self {
            tx: task_tx,
            rx: res_rx,
            next_id: 0,
        }
    }

    pub fn request_obj(&mut self, path: &str) -> JobId {
        let id = self.next_id;
        self.next_id += 1;
        let _ = self.tx.send(LoadTask::Obj {
            id,
            path: path.to_string(),
        });
        id
    }

    pub fn try_poll(&self) -> Option<LoadResult> {
        self.rx.try_recv().ok()
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(LoadTask::Shutdown);
    }
}

impl Drop for AssetLoader {
    fn drop(&mut self) {
        self.shutdown();
    }
}
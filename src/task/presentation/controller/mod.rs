use std::sync::Arc;
use rocket::fairing::AdHoc;
use crate::task::application::task_factory::TaskFactory;
use crate::task::application::task_finder::TaskFinder;
use crate::task::application::task_remover::TaskRemover;
use crate::task::application::task_updater::TaskUpdater;
use crate::task::domain::repository::task_repository::TaskRepository;
use crate::task::infrastructure::persistence::in_memory::in_memory_task_repository::InMemoryTaskRepository;

pub mod post_task;
pub mod get_tasks;
pub mod get_task;
pub mod delete_task;
pub mod patch_task;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("JSON", |rocket| async {
        let repository: Arc<dyn TaskRepository> = Arc::new(InMemoryTaskRepository::new());
        let task_factory = TaskFactory::new(Arc::clone(&repository));
        let task_finder = Arc::new(TaskFinder::new(Arc::clone(&repository)));
        let task_remover = TaskRemover::new(Arc::clone(&task_finder), Arc::clone(&repository));
        let task_updater = TaskUpdater::new(Arc::clone(&task_finder), Arc::clone(&repository));

        rocket
            .manage(task_factory)
            .manage(task_finder)
            .manage(task_remover)
            .manage(task_updater)
            .mount("/", routes![
                post_task::handle,
                get_tasks::handle,
                get_task::handle,
                delete_task::handle,
                patch_task::handle,
            ])
    })
}
use std::sync::Arc;
use rocket::fairing::AdHoc;
use crate::task::application::task_factory::TaskFactory;
use crate::task::application::task_finder::TaskFinder;
use crate::task::domain::repository::task_repository::TaskRepository;
use crate::task::infrastructure::persistence::in_memory::in_memory_task_repository::InMemoryTaskRepository;

pub mod post_task;
pub mod get_tasks;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("JSON", |rocket| async {
        let repository: Arc<dyn TaskRepository> = Arc::new(InMemoryTaskRepository::new());
        let task_factory = TaskFactory::new(Arc::clone(&repository));
        let task_finder = TaskFinder::new(repository);

        rocket
            .manage(task_factory)
            .manage(task_finder)
            .mount("/", routes![
                post_task::handle,
                get_tasks::handle,
            ])
    })
}
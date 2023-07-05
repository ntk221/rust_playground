use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(u64),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: u64) ->Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: u64, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: u64) -> anyhow::Result<()>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    id: u64,
    text: String,
    completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: u64, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<u64, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory { 
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> impl std::ops::DerefMut<Target = TodoDatas> + '_ {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> impl std::ops::Deref<Target = TodoDatas> + '_ {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload:CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as u64;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: u64) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).map(|todo| todo.clone())
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    fn update(&self, id: u64, payload: UpdateTodo) ->  anyhow::Result<Todo>{
        let mut store = self.write_store_ref();
        let todo = store
            .get(&id)
            .context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: u64) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // create
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo { text });
        assert_eq!(todo, expected);

        // find
        let todo = repository.find(todo.id).unwrap();
        assert_eq!(todo, expected);

        // all
        let todo = repository.all();
        assert_eq!(todo, vec![expected.clone()]);

        // update
        let text = "update todo text".to_string();
        let todo = repository.update(id, UpdateTodo {
            text: Some(text.clone()),
            completed: Some(true),
        }).unwrap();

        assert_eq!(
            todo, 
            Todo {
                id,
                text,
                completed: true,
            }
        );

        // delete
        let res = repository.delete(id);
        assert!(res.is_ok());
    }
}
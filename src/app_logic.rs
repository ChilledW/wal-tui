use crossterm::event::Event;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::{fs, io};

pub enum Events {
    SelectNext,
    SelectPrev,
    ChoseSelection,
}

#[derive(Debug)]
pub struct AppLogic {
    items: Vec<PathBuf>,
    displayable_items: Vec<String>,
    state: ListState,
}

impl AppLogic {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            items: Vec::new(),
            displayable_items: Vec::new(),
            state,
        }
    }

    pub fn update_items(&mut self, path_to_folder: PathBuf) -> io::Result<()> {
        let wallpapers: Vec<PathBuf> = fs::read_dir(path_to_folder)?
            .filter_map(|entry| entry.ok()) // ignore failed entries
            .map(|entry| entry.path()) // get full PathBuf
            .filter(|path| path.is_file()) // only keep files
            .collect();

        self.items = wallpapers;
        self.displayable_items = self.generate_displayable_list();

        Ok(())
    }

    fn generate_displayable_list(&mut self) -> Vec<String> {
        self.get_items()
            .iter()
            .map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<invalid name>")
                    .to_string()
            })
            .collect()
    }

    pub fn get_displayable_items(&self) -> &[String] {
        &self.displayable_items
    }

    pub fn get_items(&self) -> &[PathBuf] {
        &self.items
    }

    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn select(&mut self, index: usize) {
        // if the index is too large select the last item
        if index > self.items.len() {
            self.state.select(Some(self.items.len() - 1));
        }
    }

    fn next(&mut self) {
        self.state.select_next()
    }

    fn prev(&mut self) {
        self.state.select_previous()
    }

    pub fn handle_event(&mut self, e: Events) {
        match e {
            Events::SelectNext => self.next(),
            Events::SelectPrev => self.prev(),
            Events::ChoseSelection => {
                if let Some(i) = self.state.selected() {
                    self.select(i);
                }
            }
        }
    }
}

// #[derive(Debug)]
// pub struct App {
//     items: Vec<String>,
//     new_items: Vec<String>,
//     state: ListState,
// }
//
// impl App {
//     pub fn new(initial_list: Vec<&str>) -> Self {
//         let mut state = ListState::default();
//         state.select(Some(0));
//         let list_to_store: Vec<String> = initial_list.iter().map(|s| s.to_string()).collect();
//         let new_items: Vec<String> = Vec::new();
//
//         Self {
//             items: list_to_store,
//             new_items,
//             state,
//         }
//     }
//
//     pub fn next(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             _ => 0,
//         };
//         self.state.select(Some(i));
//     }
//
//     pub fn prev(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len()
//                 } else {
//                     i - 1
//                 }
//             }
//             _ => 0,
//         };
//         self.state.select(Some(i));
//     }
//
//     pub fn select(&mut self) {
//         if let Some(i) = self.state.selected() {
//             self.new_items.push(self.items[i].clone());
//         }
//     }
// }

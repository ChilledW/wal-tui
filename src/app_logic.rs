#[derive(Debug)]
pub mod AppLogic {
    pub struct App {
        items: Vec<String>,
        new_items: Vec<String>,
        state: ListState,
    }

    impl App {
        fn new(initial_list: Vec<&str>) -> Self {
            let mut state = ListState::default();
            state.select(Some(0));
            let list_to_store: Vec<String> = initial_list.iter().map(|s| s.to_string()).collect();
            let new_items: Vec<String> = Vec::new();

            Self {
                items: list_to_store,
                new_items,
                state,
            }
        }

        fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                _ => 0,
            };
            self.state.select(Some(i));
        }

        fn prev(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len()
                    } else {
                        i - 1
                    }
                }
                _ => 0,
            };
            self.state.select(Some(i));
        }

        fn select(&mut self) {
            if let Some(i) = self.state.selected() {
                self.new_items.push(self.items[i].clone());
            }
        }
    }
}

use crate::models::App;

impl App {
    /// Navigate to the next service in the service list
    pub fn service_next(&mut self) {
        if self.available_services.is_empty() {
            return;
        }
        let i = match self.service_list_state.selected() {
            Some(i) => {
                if i >= self.available_services.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.service_list_state.select(Some(i));
    }

    /// Navigate to the previous service in the service list
    pub fn service_previous(&mut self) {
        if self.available_services.is_empty() {
            return;
        }
        let i = match self.service_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.available_services.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.service_list_state.select(Some(i));
    }

    /// Navigate to the next instance in the instance list
    pub fn next(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.instances.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Navigate to the previous instance in the instance list
    pub fn previous(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.instances.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

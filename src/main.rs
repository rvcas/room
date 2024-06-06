use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

struct State {
    tabs: Vec<TabInfo>,
    filter: String,
    selected: Option<usize>,
    ignore_case: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tabs: Vec::default(),
            filter: String::default(),
            selected: None,
            ignore_case: false,
        }
    }
}

impl State {
    fn filter(&self, tab: &&TabInfo) -> bool {
        if self.ignore_case {
            tab.name.to_lowercase() == self.filter.to_lowercase()
                || tab
                    .name
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
        } else {
            tab.name == self.filter || tab.name.contains(&self.filter)
        }
    }

    fn viewable_tabs_iter(&self) -> impl Iterator<Item = &TabInfo> {
        self.tabs.iter().filter(|tab| self.filter(tab))
    }

    fn viewable_tabs(&self) -> Vec<&TabInfo> {
        self.viewable_tabs_iter().collect()
    }

    fn reset_selection(&mut self) {
        let tabs = self.viewable_tabs();

        if tabs.is_empty() {
            self.selected = None
        } else if let Some(tab) = tabs.first() {
            self.selected = Some(tab.position)
        }
    }

    fn select_down(&mut self) {
        let tabs = self.tabs.iter().filter(|tab| self.filter(tab));

        let mut can_select = false;
        let mut first = None;
        for TabInfo { position, .. } in tabs {
            if first.is_none() {
                first.replace(position);
            }

            if can_select {
                self.selected = Some(*position);
                return;
            } else if Some(*position) == self.selected {
                can_select = true;
            }
        }

        if let Some(position) = first {
            self.selected = Some(*position)
        }
    }

    fn select_up(&mut self) {
        let tabs = self.tabs.iter().filter(|tab| self.filter(tab)).rev();

        let mut can_select = false;
        let mut last = None;
        for TabInfo { position, .. } in tabs {
            if last.is_none() {
                last.replace(position);
            }

            if can_select {
                self.selected = Some(*position);
                return;
            } else if Some(*position) == self.selected {
                can_select = true;
            }
        }

        if let Some(position) = last {
            self.selected = Some(*position)
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        // we need the ReadApplicationState permission to receive the ModeUpdate and TabUpdate
        // events
        // we need the ChangeApplicationState permission to Change Zellij state (Panes, Tabs and UI)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        self.ignore_case = match configuration.get("ignore_case" as &str) {
            Some(value) => value.trim().parse().unwrap(),
            None => true,
        };

        subscribe(&[EventType::TabUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::TabUpdate(tab_info) => {
                self.selected =
                    tab_info.iter().find_map(
                        |tab| {
                            if tab.active {
                                Some(tab.position)
                            } else {
                                None
                            }
                        },
                    );

                self.tabs = tab_info;
                should_render = true;
            }

            Event::Key(Key::Esc | Key::Ctrl('c')) => {
                close_focus();
            }

            Event::Key(Key::Down | Key::BackTab) => {
                self.select_down();

                should_render = true;
            }
            Event::Key(Key::Up | Key::Ctrl('k')) => {
                self.select_up();

                should_render = true;
            }
            Event::Key(Key::Char('\n')) => {
                let tab = self
                    .tabs
                    .iter()
                    .find(|tab| Some(tab.position) == self.selected);

                if let Some(tab) = tab {
                    close_focus();
                    switch_tab_to(tab.position as u32 + 1);
                }
            }
            Event::Key(Key::Backspace) => {
                self.filter.pop();

                self.reset_selection();

                should_render = true;
            }
            Event::Key(Key::Char(c)) if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                self.filter.push(c);

                self.reset_selection();

                should_render = true;
            }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!(
            "{} {}",
            ">".cyan().bold(),
            if self.filter.is_empty() {
                "(filter)".dimmed().italic().to_string()
            } else {
                self.filter.dimmed().italic().to_string()
            }
        );

        println!(
            "{}",
            self.viewable_tabs_iter()
                .map(|tab| {
                    let row = if tab.active {
                        format!("{} - {}", tab.position + 1, tab.name)
                            .red()
                            .bold()
                            .to_string()
                    } else {
                        format!("{} - {}", tab.position + 1, tab.name)
                    };

                    if Some(tab.position) == self.selected {
                        row.on_cyan().to_string()
                    } else {
                        row
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

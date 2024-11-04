use owo_colors::{AnsiColors, OwoColorize};
use std::collections::BTreeMap;
use std::fmt::Write;
use zellij_tile::prelude::*;

struct State {
    tabs: Vec<TabInfo>,
    filter: String,
    selected: Option<usize>,
    ignore_case: bool,
    quick_jump: bool,

    selection_color: AnsiColors,
    apply_selection_for_foreground_instead: bool,
    active_tab_color: Option<AnsiColors>,
    underline_active: bool,
    apply_active_color_for_background_instead: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tabs: Default::default(),
            filter: Default::default(),
            selected: Default::default(),
            ignore_case: true,
            quick_jump: false,
            selection_color: AnsiColors::Yellow,
            active_tab_color: None,
            underline_active: true,
            apply_selection_for_foreground_instead: true,
            apply_active_color_for_background_instead: false,
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
    fn load(&mut self, mut configuration: BTreeMap<String, String>) {
        // we need the ReadApplicationState permission to receive the ModeUpdate and TabUpdate
        // events
        // we need the ChangeApplicationState permission to Change Zellij state (Panes, Tabs and UI)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        if let Some(value) = configuration.remove("ignore_case") {
            self.ignore_case = value.trim().parse().unwrap_or_else(|_| {
                panic!(
                    "'ingnore_case' config value must be 'true' or 'false', but it's \"{value}\""
                )
            });
        };

        if let Some(quick_jump) = configuration.remove("quick_jump") {
            self.quick_jump = quick_jump.trim().parse().unwrap_or_else(|_| {
                panic!("'quick_jump' config value must be 'true' or 'false', but it's \"{quick_jump}\"")
            });
        }

        if let Some(color) = configuration.remove("selection_color") {
            // TODO: validate input
            self.selection_color = color.trim().into();
        }

        if let Some(x) = configuration.remove("apply_selection_accent_to") {
            match x.as_str() {
                "background" | "bg" => self.apply_selection_for_foreground_instead = false,
                "foreground" | "fg" => self.apply_selection_for_foreground_instead = true,
                _ => panic!("'apply_selection_accent_to' config value must be 'fg', 'foreground', 'bg' or 'background', but it's \"{x}\""),
            }
        }

        if let Some(color) = configuration.remove("active_tab_color") {
            // TODO: validate input
            let temp = color.trim();
            if temp == "none" {
                self.active_tab_color = None;
            } else {
                self.active_tab_color = Some(temp.into());
            }
        }

        if let Some(x) = configuration.remove("apply_tab_color_to") {
            match x.as_str() {
                "background" | "bg" => self.apply_active_color_for_background_instead = true,
                "foreground" | "fg" => self.apply_active_color_for_background_instead = false,
                _ => panic!("'apply_tab_color_to' config value must be 'fg', 'foreground', 'bg' or 'background', but it's \"{x}\""),
            }
        }

        if let Some(value) = configuration.remove("underline_active") {
            self.underline_active = value.trim().parse().unwrap_or_else(|_| {
                panic!(
                    "'underline_active' config value must be 'true' or 'false', but it's \"{value}\""
                )
            });
        };

        if !configuration.is_empty() {
            let stringified_map = configuration
                .iter()
                .fold(String::new(), |mut output, (k, v)| {
                    let _ = writeln!(output, "('{k}': '{v}')\n");
                    output
                });

            eprintln!("WARNING: The user added a config entry that isn't used.");

            eprint!("{stringified_map}");
        }

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

            Event::Key(key) => match key.bare_key {
                BareKey::Esc => {
                    close_focus();
                }
                BareKey::Char('c') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    close_focus();
                }

                BareKey::Down => {
                    self.select_down();

                    should_render = true;
                }
                BareKey::Tab if key.has_no_modifiers() => {
                    self.select_down();

                    should_render = true;
                }
                BareKey::Char('n') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_down();

                    should_render = true;
                }

                BareKey::Up => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Tab if key.has_modifiers(&[KeyModifier::Shift]) => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Char('k') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Char('p') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_up();

                    should_render = true;
                }

                BareKey::Enter => {
                    let tab = self
                        .tabs
                        .iter()
                        .find(|tab| Some(tab.position) == self.selected);

                    if let Some(tab) = tab {
                        close_focus();
                        switch_tab_to(tab.position as u32 + 1);
                    }
                }
                BareKey::Backspace => {
                    self.filter.pop();

                    self.reset_selection();

                    should_render = true;
                }

                BareKey::Char(c) if c.is_ascii_digit() && self.quick_jump => {
                    close_focus();
                    switch_tab_to(c.to_digit(10).unwrap());
                }

                BareKey::Char(c) if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                    self.filter.push(c);

                    self.reset_selection();

                    should_render = true;
                }
                _ => (),
            },

            _ => (),
        };

        should_render
    }

    #[allow(unused_mut)]
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
                    let mut row = {
                        let mut position = format!("{}", tab.position + 1);
                        let mut name = tab.name.to_string();
                        // Change components

                        if tab.active && self.underline_active {
                            name = name.underline().to_string();
                        }
                        if self
                            .selected
                            .is_some_and(|selected_tab| tab.position == selected_tab)
                        {
                            if self.apply_selection_for_foreground_instead {
                                name = name.color(self.selection_color).to_string();
                            } else {
                                name = name.on_color(self.selection_color).to_string();
                            }
                        }

                        format!("{} - {}", position, name)
                    };
                    // Changes for all row
                    if tab.active {
                        if let Some(color) = self.active_tab_color {
                            if self.apply_active_color_for_background_instead {
                                row = row.on_color(color).to_string();
                            } else {
                                row = row.color(color).to_string();
                            }
                        }
                    }
                    row
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

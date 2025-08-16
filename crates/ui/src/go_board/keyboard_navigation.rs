use crate::go_board::types::Vertex;
use gpui::KeyDownEvent;

/// Keyboard navigation handler for accessibility and efficient selection management
/// Provides arrow key navigation, selection shortcuts, and accessibility features
pub struct KeyboardNavigation {
    pub current_focus: Option<Vertex>,
    pub board_width: usize,
    pub board_height: usize,
}

impl KeyboardNavigation {
    pub fn new(board_width: usize, board_height: usize) -> Self {
        Self {
            current_focus: None,
            board_width,
            board_height,
        }
    }

    /// Handles keyboard input and returns the updated focus vertex
    pub fn handle_key_event(&mut self, key: &KeyDownEvent) -> Option<NavigationAction> {
        match key.keystroke.key.as_str() {
            "ArrowUp" => self.move_focus(0, -1),
            "ArrowDown" => self.move_focus(0, 1),
            "ArrowLeft" => self.move_focus(-1, 0),
            "ArrowRight" => self.move_focus(1, 0),
            "Enter" | " " => {
                if let Some(vertex) = &self.current_focus {
                    Some(NavigationAction::Select(vertex.clone()))
                } else {
                    None
                }
            }
            "Escape" => Some(NavigationAction::ClearSelection),
            "Home" => {
                self.current_focus = Some(Vertex::new(0, 0));
                Some(NavigationAction::FocusChanged(Vertex::new(0, 0)))
            }
            "End" => {
                let end_vertex = Vertex::new(self.board_width - 1, self.board_height - 1);
                self.current_focus = Some(end_vertex.clone());
                Some(NavigationAction::FocusChanged(end_vertex))
            }
            _ => None,
        }
    }

    /// Moves focus in the specified direction
    fn move_focus(&mut self, dx: i32, dy: i32) -> Option<NavigationAction> {
        let default_vertex = Vertex::new(0, 0);
        let current = self.current_focus.as_ref().unwrap_or(&default_vertex);

        let new_x = (current.x as i32 + dx)
            .max(0)
            .min(self.board_width as i32 - 1) as usize;
        let new_y = (current.y as i32 + dy)
            .max(0)
            .min(self.board_height as i32 - 1) as usize;

        let new_vertex = Vertex::new(new_x, new_y);

        if self.current_focus.as_ref() != Some(&new_vertex) {
            self.current_focus = Some(new_vertex.clone());
            Some(NavigationAction::FocusChanged(new_vertex))
        } else {
            None // No change in focus
        }
    }

    /// Sets the current focus vertex programmatically
    pub fn set_focus(&mut self, vertex: Option<Vertex>) -> Option<NavigationAction> {
        if let Some(v) = vertex {
            if v.x < self.board_width && v.y < self.board_height {
                self.current_focus = Some(v.clone());
                Some(NavigationAction::FocusChanged(v))
            } else {
                None
            }
        } else {
            self.current_focus = None;
            Some(NavigationAction::ClearFocus)
        }
    }

    /// Gets the current focus vertex
    pub fn get_focus(&self) -> Option<Vertex> {
        self.current_focus.clone()
    }

    /// Updates board dimensions for navigation bounds
    pub fn update_bounds(&mut self, width: usize, height: usize) {
        self.board_width = width;
        self.board_height = height;

        // Ensure current focus is still valid
        if let Some(focus) = &self.current_focus {
            if focus.x >= width || focus.y >= height {
                self.current_focus = None;
            }
        }
    }
}

/// Actions that can result from keyboard navigation
#[derive(Clone, Debug, PartialEq)]
pub enum NavigationAction {
    FocusChanged(Vertex),
    Select(Vertex),
    ClearSelection,
    ClearFocus,
}

/// Keyboard-accessible selection manager that combines navigation with selection state
pub struct AccessibleSelectionManager {
    pub navigation: KeyboardNavigation,
    pub selected_vertices: Vec<Vertex>,
    pub multi_select_mode: bool,
}

impl AccessibleSelectionManager {
    pub fn new(board_width: usize, board_height: usize) -> Self {
        Self {
            navigation: KeyboardNavigation::new(board_width, board_height),
            selected_vertices: Vec::new(),
            multi_select_mode: false,
        }
    }

    /// Handles keyboard events with selection state management
    pub fn handle_key_event(&mut self, key: &KeyDownEvent) -> Vec<SelectionUpdate> {
        let mut updates = Vec::new();

        // Check for modifier keys
        let shift_held = key.keystroke.modifiers.shift;
        let ctrl_held = key.keystroke.modifiers.control || key.keystroke.modifiers.platform;

        // Handle multi-select mode toggle
        if ctrl_held && !self.multi_select_mode {
            self.multi_select_mode = true;
        }

        if let Some(action) = self.navigation.handle_key_event(key) {
            match action {
                NavigationAction::FocusChanged(vertex) => {
                    updates.push(SelectionUpdate::FocusChanged(vertex));
                }
                NavigationAction::Select(vertex) => {
                    if self.multi_select_mode || ctrl_held {
                        // Toggle selection in multi-select mode
                        if let Some(pos) = self.selected_vertices.iter().position(|v| *v == vertex)
                        {
                            self.selected_vertices.remove(pos);
                            updates.push(SelectionUpdate::Deselected(vertex));
                        } else {
                            self.selected_vertices.push(vertex.clone());
                            updates.push(SelectionUpdate::Selected(vertex));
                        }
                    } else if shift_held {
                        // Range selection
                        if let Some(last_selected) = self.selected_vertices.last() {
                            let range = self.calculate_selection_range(last_selected, &vertex);
                            self.selected_vertices.extend(range.clone());
                            updates.push(SelectionUpdate::RangeSelected(range));
                        } else {
                            self.selected_vertices = vec![vertex.clone()];
                            updates.push(SelectionUpdate::Selected(vertex));
                        }
                    } else {
                        // Single selection (clear others)
                        self.selected_vertices = vec![vertex.clone()];
                        updates.push(SelectionUpdate::SelectionReplaced(vec![vertex]));
                    }
                }
                NavigationAction::ClearSelection => {
                    self.selected_vertices.clear();
                    updates.push(SelectionUpdate::AllDeselected);
                }
                NavigationAction::ClearFocus => {
                    updates.push(SelectionUpdate::FocusCleared);
                }
            }
        }

        // Reset multi-select mode when no modifier keys are held
        if !ctrl_held && !shift_held {
            self.multi_select_mode = false;
        }

        updates
    }

    /// Calculates vertices in a rectangular range between two points
    fn calculate_selection_range(&self, start: &Vertex, end: &Vertex) -> Vec<Vertex> {
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        let mut range = Vec::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                range.push(Vertex::new(x, y));
            }
        }
        range
    }

    /// Gets current selection state
    pub fn get_selected_vertices(&self) -> &[Vertex] {
        &self.selected_vertices
    }

    /// Gets current focus
    pub fn get_focus(&self) -> Option<Vertex> {
        self.navigation.get_focus()
    }
}

/// Updates that result from selection management
#[derive(Clone, Debug, PartialEq)]
pub enum SelectionUpdate {
    FocusChanged(Vertex),
    FocusCleared,
    Selected(Vertex),
    Deselected(Vertex),
    RangeSelected(Vec<Vertex>),
    SelectionReplaced(Vec<Vertex>),
    AllDeselected,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Keystroke, Modifiers};

    fn create_key_event(key: &str, modifiers: Modifiers) -> KeyDownEvent {
        KeyDownEvent {
            keystroke: Keystroke {
                key: key.to_string(),
                modifiers,
                ime_key: None,
            },
            is_held: false,
        }
    }

    #[test]
    fn test_keyboard_navigation_creation() {
        let nav = KeyboardNavigation::new(19, 19);
        assert_eq!(nav.board_width, 19);
        assert_eq!(nav.board_height, 19);
        assert_eq!(nav.current_focus, None);
    }

    #[test]
    fn test_focus_movement() {
        let mut nav = KeyboardNavigation::new(9, 9);
        nav.set_focus(Some(Vertex::new(4, 4)));

        // Test arrow key movements
        let key_up = create_key_event("ArrowUp", Modifiers::default());
        let action = nav.handle_key_event(&key_up);
        assert_eq!(
            action,
            Some(NavigationAction::FocusChanged(Vertex::new(4, 3)))
        );

        let key_right = create_key_event("ArrowRight", Modifiers::default());
        let action = nav.handle_key_event(&key_right);
        assert_eq!(
            action,
            Some(NavigationAction::FocusChanged(Vertex::new(5, 3)))
        );
    }

    #[test]
    fn test_boundary_handling() {
        let mut nav = KeyboardNavigation::new(3, 3);
        nav.set_focus(Some(Vertex::new(0, 0)));

        // Test movement at boundaries
        let key_up = create_key_event("ArrowUp", Modifiers::default());
        let action = nav.handle_key_event(&key_up);
        assert_eq!(action, None); // Should not move beyond boundary

        let key_left = create_key_event("ArrowLeft", Modifiers::default());
        let action = nav.handle_key_event(&key_left);
        assert_eq!(action, None); // Should not move beyond boundary
    }

    #[test]
    fn test_home_end_keys() {
        let mut nav = KeyboardNavigation::new(9, 9);

        let key_home = create_key_event("Home", Modifiers::default());
        let action = nav.handle_key_event(&key_home);
        assert_eq!(
            action,
            Some(NavigationAction::FocusChanged(Vertex::new(0, 0)))
        );

        let key_end = create_key_event("End", Modifiers::default());
        let action = nav.handle_key_event(&key_end);
        assert_eq!(
            action,
            Some(NavigationAction::FocusChanged(Vertex::new(8, 8)))
        );
    }

    #[test]
    fn test_accessible_selection_manager() {
        let mut manager = AccessibleSelectionManager::new(9, 9);

        // Test single selection
        let enter_key = create_key_event("Enter", Modifiers::default());

        // First set focus
        manager.navigation.set_focus(Some(Vertex::new(2, 2)));

        let updates = manager.handle_key_event(&enter_key);
        assert_eq!(updates.len(), 1);
        assert_eq!(
            updates[0],
            SelectionUpdate::SelectionReplaced(vec![Vertex::new(2, 2)])
        );
        assert_eq!(manager.get_selected_vertices().len(), 1);
    }

    #[test]
    fn test_multi_select_mode() {
        let mut manager = AccessibleSelectionManager::new(9, 9);

        // Set focus and make first selection
        manager.navigation.set_focus(Some(Vertex::new(2, 2)));
        let enter_key = create_key_event("Enter", Modifiers::default());
        manager.handle_key_event(&enter_key);

        // Move focus and make second selection with Ctrl
        manager.navigation.set_focus(Some(Vertex::new(3, 3)));
        let ctrl_enter = create_key_event(
            "Enter",
            Modifiers {
                control: true,
                ..Default::default()
            },
        );
        let updates = manager.handle_key_event(&ctrl_enter);

        assert_eq!(manager.get_selected_vertices().len(), 2);
        assert!(updates
            .iter()
            .any(|u| matches!(u, SelectionUpdate::Selected(_))));
    }

    #[test]
    fn test_range_selection() {
        let mut manager = AccessibleSelectionManager::new(9, 9);

        // Make initial selection
        manager.navigation.set_focus(Some(Vertex::new(1, 1)));
        let enter_key = create_key_event("Enter", Modifiers::default());
        manager.handle_key_event(&enter_key);

        // Move focus and make range selection with Shift
        manager.navigation.set_focus(Some(Vertex::new(2, 2)));
        let shift_enter = create_key_event(
            "Enter",
            Modifiers {
                shift: true,
                ..Default::default()
            },
        );
        let updates = manager.handle_key_event(&shift_enter);

        // Should have range selection
        assert!(manager.get_selected_vertices().len() > 2);
        assert!(updates
            .iter()
            .any(|u| matches!(u, SelectionUpdate::RangeSelected(_))));
    }

    #[test]
    fn test_clear_selection() {
        let mut manager = AccessibleSelectionManager::new(9, 9);

        // Make some selections
        manager.selected_vertices = vec![Vertex::new(1, 1), Vertex::new(2, 2)];

        let escape_key = create_key_event("Escape", Modifiers::default());
        let updates = manager.handle_key_event(&escape_key);

        assert_eq!(manager.get_selected_vertices().len(), 0);
        assert_eq!(updates, vec![SelectionUpdate::AllDeselected]);
    }
}

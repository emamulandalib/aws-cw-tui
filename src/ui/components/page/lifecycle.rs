/// Page lifecycle management for consistent initialization and cleanup
///
/// This module provides lifecycle management for pages to ensure proper
/// initialization, state management, and cleanup.
use std::time::Instant;

/// Page lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleState {
    /// Page is being created
    Initializing,
    /// Page is ready and active
    Active,
    /// Page is being updated
    Updating,
    /// Page is being destroyed
    Destroying,
    /// Page encountered an error
    Error(String),
}

/// Page lifecycle manager
#[derive(Debug, Clone)]
pub struct PageLifecycle {
    /// Current lifecycle state
    state: LifecycleState,
    /// When the page was created
    created_at: Instant,
    /// When the page was last updated
    last_updated: Option<Instant>,
    /// Number of updates performed
    update_count: u32,
    /// Whether the page has been initialized
    initialized: bool,
}

impl PageLifecycle {
    /// Create a new page lifecycle
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Initializing,
            created_at: Instant::now(),
            last_updated: None,
            update_count: 0,
            initialized: false,
        }
    }

    /// Get the current lifecycle state
    pub fn state(&self) -> &LifecycleState {
        &self.state
    }

    /// Set the lifecycle state
    pub fn set_state(&mut self, state: LifecycleState) {
        if matches!(state, LifecycleState::Active) && !self.initialized {
            self.initialized = true;
        }
        self.state = state;
    }

    /// Mark the page as initialized and active
    pub fn mark_initialized(&mut self) {
        self.state = LifecycleState::Active;
        self.initialized = true;
    }

    /// Mark the page as updated
    pub fn mark_updated(&mut self) {
        self.last_updated = Some(Instant::now());
        self.update_count += 1;
        if self.state == LifecycleState::Updating {
            self.state = LifecycleState::Active;
        }
    }

    /// Mark the page as updating
    pub fn mark_updating(&mut self) {
        self.state = LifecycleState::Updating;
    }

    /// Mark the page as being destroyed
    pub fn mark_destroying(&mut self) {
        self.state = LifecycleState::Destroying;
    }

    /// Set an error state
    pub fn set_error(&mut self, error: String) {
        self.state = LifecycleState::Error(error);
    }

    /// Check if the page is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Check if the page is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, LifecycleState::Active)
    }

    /// Check if the page is in an error state
    pub fn is_error(&self) -> bool {
        matches!(self.state, LifecycleState::Error(_))
    }

    /// Get the error message if in error state
    pub fn error_message(&self) -> Option<&String> {
        match &self.state {
            LifecycleState::Error(msg) => Some(msg),
            _ => None,
        }
    }

    /// Get the age of the page
    pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }

    /// Get the time since last update
    pub fn time_since_update(&self) -> Option<std::time::Duration> {
        self.last_updated.map(|t| t.elapsed())
    }

    /// Get the number of updates
    pub fn update_count(&self) -> u32 {
        self.update_count
    }
}

impl Default for PageLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

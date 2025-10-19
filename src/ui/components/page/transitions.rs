/// Page transition system for consistent navigation patterns
///
/// This module provides transition effects and navigation patterns
/// for moving between pages in the application.
use std::time::{Duration, Instant};

/// Types of page transitions
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    /// No transition effect
    None,
    /// Fade in/out transition
    Fade,
    /// Slide transition (left, right, up, down)
    Slide(SlideDirection),
    /// Push transition (new page pushes old page out)
    Push(SlideDirection),
}

/// Direction for slide and push transitions
#[derive(Debug, Clone, PartialEq)]
pub enum SlideDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Page transition state
#[derive(Debug, Clone)]
pub struct PageTransition {
    /// Type of transition
    transition_type: TransitionType,
    /// Duration of the transition
    duration: Duration,
    /// When the transition started
    start_time: Option<Instant>,
    /// Whether the transition is currently active
    active: bool,
    /// Progress of the transition (0.0 to 1.0)
    progress: f32,
}

impl PageTransition {
    /// Create a new page transition
    pub fn new(transition_type: TransitionType, duration: Duration) -> Self {
        Self {
            transition_type,
            duration,
            start_time: None,
            active: false,
            progress: 0.0,
        }
    }

    /// Create a fade transition
    pub fn fade(duration: Duration) -> Self {
        Self::new(TransitionType::Fade, duration)
    }

    /// Create a slide transition
    pub fn slide(direction: SlideDirection, duration: Duration) -> Self {
        Self::new(TransitionType::Slide(direction), duration)
    }

    /// Create a push transition
    pub fn push(direction: SlideDirection, duration: Duration) -> Self {
        Self::new(TransitionType::Push(direction), duration)
    }

    /// Create no transition (instant)
    pub fn none() -> Self {
        Self::new(TransitionType::None, Duration::from_millis(0))
    }

    /// Start the transition
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.active = true;
        self.progress = 0.0;
    }

    /// Update the transition progress
    pub fn update(&mut self) {
        if !self.active {
            return;
        }

        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();

            if elapsed >= self.duration {
                // Transition complete
                self.progress = 1.0;
                self.active = false;
            } else {
                // Calculate progress (0.0 to 1.0)
                self.progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

                // Apply easing function for smoother transitions
                self.progress = self.ease_in_out_cubic(self.progress);
            }
        }
    }

    /// Check if the transition is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if the transition is complete
    pub fn is_complete(&self) -> bool {
        !self.active && self.progress >= 1.0
    }

    /// Get the current progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Get the transition type
    pub fn transition_type(&self) -> &TransitionType {
        &self.transition_type
    }

    /// Reset the transition
    pub fn reset(&mut self) {
        self.start_time = None;
        self.active = false;
        self.progress = 0.0;
    }

    /// Ease-in-out cubic function for smooth transitions
    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

impl Default for PageTransition {
    fn default() -> Self {
        Self::none()
    }
}

/// Page transition manager for handling multiple transitions
#[derive(Debug)]
pub struct TransitionManager {
    /// Current active transition
    current_transition: Option<PageTransition>,
    /// Default transition for page changes
    default_transition: PageTransition,
}

impl TransitionManager {
    /// Create a new transition manager
    pub fn new() -> Self {
        Self {
            current_transition: None,
            default_transition: PageTransition::fade(Duration::from_millis(200)),
        }
    }

    /// Set the default transition
    pub fn set_default_transition(&mut self, transition: PageTransition) {
        self.default_transition = transition;
    }

    /// Start a transition
    pub fn start_transition(&mut self, transition: PageTransition) {
        let mut transition = transition;
        transition.start();
        self.current_transition = Some(transition);
    }

    /// Start the default transition
    pub fn start_default_transition(&mut self) {
        let mut transition = self.default_transition.clone();
        transition.start();
        self.current_transition = Some(transition);
    }

    /// Update the current transition
    pub fn update(&mut self) {
        if let Some(transition) = &mut self.current_transition {
            transition.update();

            if transition.is_complete() {
                self.current_transition = None;
            }
        }
    }

    /// Check if a transition is active
    pub fn is_transitioning(&self) -> bool {
        self.current_transition
            .as_ref()
            .map(|t| t.is_active())
            .unwrap_or(false)
    }

    /// Get the current transition
    pub fn current_transition(&self) -> Option<&PageTransition> {
        self.current_transition.as_ref()
    }

    /// Clear the current transition
    pub fn clear_transition(&mut self) {
        self.current_transition = None;
    }
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

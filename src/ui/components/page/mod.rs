pub mod lifecycle;
pub mod overlays;
pub mod page_manager;
/// Page component system for standardized UI structure
///
/// This module provides a consistent page component system that all screens
/// in the application should use. It includes:
/// - Page trait for consistent page structure
/// - StandardPageLayout for header/content/footer layout
/// - PageLifecycle for initialization and cleanup
/// - Loading and error overlay support
/// - Page transition system
/// - PageManager for coordinating all pages
pub mod page_trait;
pub mod pages;
pub mod standard_layout;
pub mod transitions;

pub use lifecycle::{LifecycleState, PageLifecycle};
pub use overlays::{ErrorOverlay, LoadingOverlay, OverlayType};
pub use page_manager::PageManager;
pub use page_trait::{EventResult, Page, PageContent, PageResult};
pub use standard_layout::StandardPageLayout;
pub use transitions::{PageTransition, TransitionType};

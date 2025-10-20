use crate::ui::components::list_styling::border_factory;
use crate::ui::themes::UnifiedTheme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

/// Universal box component for consistent rendering across all non-chart UI elements
pub struct UniversalBox {
    title: Option<String>,
    borders: Borders,
    is_focused: bool,
    content_type: BoxContent,
    theme: UnifiedTheme,
}

/// Different types of content that can be rendered inside the box
pub enum BoxContent {
    /// Simple text content
    Text {
        text: String,
        alignment: Alignment,
        wrap: bool,
        style_override: Option<Style>,
    },
    /// List content with state management
    List {
        items: Vec<ListItem<'static>>,
        list_state: Option<ListState>,
        highlight_style: Option<Style>,
    },
    /// Multiple paragraphs with different styles
    MultiText { paragraphs: Vec<TextParagraph> },
    /// Custom content (for complex layouts)
    Custom {
        render_fn: Box<dyn Fn(&mut Frame, Rect, &UnifiedTheme)>,
    },
    /// Loading state
    Loading { message: String },
    /// Error state
    Error { message: String },
    /// Empty state
    Empty {
        message: String,
        suggestion: Option<String>,
    },
}

/// Text paragraph with individual styling
pub struct TextParagraph {
    pub text: String,
    pub style: Option<Style>,
    pub alignment: Alignment,
}

impl UniversalBox {
    /// Create a new universal box with default settings
    pub fn new(theme: UnifiedTheme) -> Self {
        Self {
            title: None,
            borders: Borders::ALL,
            is_focused: false,
            content_type: BoxContent::Empty {
                message: "No content".to_string(),
                suggestion: None,
            },
            theme,
        }
    }

    /// Set the title of the box
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the borders of the box
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Set the focus state of the box
    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    /// Set simple text content
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.content_type = BoxContent::Text {
            text: text.into(),
            alignment: Alignment::Left,
            wrap: false,
            style_override: None,
        };
        self
    }

    /// Set text content with custom alignment
    pub fn text_aligned<S: Into<String>>(mut self, text: S, alignment: Alignment) -> Self {
        self.content_type = BoxContent::Text {
            text: text.into(),
            alignment,
            wrap: false,
            style_override: None,
        };
        self
    }

    /// Set wrapped text content
    pub fn text_wrapped<S: Into<String>>(mut self, text: S) -> Self {
        self.content_type = BoxContent::Text {
            text: text.into(),
            alignment: Alignment::Left,
            wrap: true,
            style_override: None,
        };
        self
    }

    /// Set text content with custom style
    pub fn text_styled<S: Into<String>>(mut self, text: S, style: Style) -> Self {
        self.content_type = BoxContent::Text {
            text: text.into(),
            alignment: Alignment::Left,
            wrap: false,
            style_override: Some(style),
        };
        self
    }

    /// Set list content
    pub fn list(mut self, items: Vec<ListItem<'static>>) -> Self {
        self.content_type = BoxContent::List {
            items,
            list_state: None,
            highlight_style: None,
        };
        self
    }

    /// Set list content with state management
    pub fn list_with_state(mut self, items: Vec<ListItem<'static>>, list_state: ListState) -> Self {
        self.content_type = BoxContent::List {
            items,
            list_state: Some(list_state),
            highlight_style: None,
        };
        self
    }

    /// Set list content with custom highlight style
    pub fn list_styled(mut self, items: Vec<ListItem<'static>>, highlight_style: Style) -> Self {
        self.content_type = BoxContent::List {
            items,
            list_state: None,
            highlight_style: Some(highlight_style),
        };
        self
    }

    /// Set multiple text paragraphs
    pub fn multi_text(mut self, paragraphs: Vec<TextParagraph>) -> Self {
        self.content_type = BoxContent::MultiText { paragraphs };
        self
    }

    /// Set loading state
    pub fn loading<S: Into<String>>(mut self, message: S) -> Self {
        self.content_type = BoxContent::Loading {
            message: message.into(),
        };
        self
    }

    /// Set error state
    pub fn error<S: Into<String>>(mut self, message: S) -> Self {
        self.content_type = BoxContent::Error {
            message: message.into(),
        };
        self
    }

    /// Set empty state
    pub fn empty<S: Into<String>>(mut self, message: S) -> Self {
        self.content_type = BoxContent::Empty {
            message: message.into(),
            suggestion: None,
        };
        self
    }

    /// Set empty state with suggestion
    pub fn empty_with_suggestion<S: Into<String>, T: Into<String>>(
        mut self,
        message: S,
        suggestion: T,
    ) -> Self {
        self.content_type = BoxContent::Empty {
            message: message.into(),
            suggestion: Some(suggestion.into()),
        };
        self
    }

    /// Set custom content with render function
    pub fn custom<F>(mut self, render_fn: F) -> Self
    where
        F: Fn(&mut Frame, Rect, &UnifiedTheme) + 'static,
    {
        self.content_type = BoxContent::Custom {
            render_fn: Box::new(render_fn),
        };
        self
    }

    /// Render the box with its content
    pub fn render(self, f: &mut Frame, area: Rect) {
        self.render_with_state(f, area, None);
    }

    /// Render the box with optional mutable list state
    pub fn render_with_state(self, f: &mut Frame, area: Rect, list_state: Option<&mut ListState>) {
        // Use centralized border factory for theme-consistent border styling
        let border_style = border_factory::create_theme_border_style(&self.theme, self.is_focused);

        // Create the block
        let mut block = Block::default()
            .borders(self.borders)
            .border_style(border_style);

        // Add title if present
        if let Some(title) = &self.title {
            block = block.title(title.clone());
        }

        // Get inner area for content
        let inner_area = block.inner(area);

        // Render the block
        f.render_widget(block, area);

        // Render content based on type
        match self.content_type {
            BoxContent::Text {
                text,
                alignment,
                wrap,
                style_override,
            } => {
                let default_style = Style::default().fg(self.theme.primary);
                let style = style_override.unwrap_or(default_style);

                let mut paragraph = Paragraph::new(text).style(style).alignment(alignment);

                if wrap {
                    paragraph = paragraph.wrap(Wrap { trim: true });
                }

                f.render_widget(paragraph, inner_area);
            }

            BoxContent::List {
                items,
                list_state: internal_state,
                highlight_style,
            } => {
                let default_highlight = Style::default()
                    .fg(self.theme.selected)
                    .add_modifier(Modifier::BOLD);
                let highlight = highlight_style.unwrap_or(default_highlight);

                let list_widget = List::new(items)
                    .style(Style::default().fg(self.theme.primary))
                    .highlight_style(highlight);

                // Use external state if provided, otherwise use internal state
                if let Some(external_state) = list_state {
                    f.render_stateful_widget(list_widget, inner_area, external_state);
                } else if let Some(mut internal_state) = internal_state {
                    f.render_stateful_widget(list_widget, inner_area, &mut internal_state);
                } else {
                    f.render_widget(list_widget, inner_area);
                }
            }

            BoxContent::MultiText { paragraphs } => {
                // Calculate space for each paragraph
                let paragraph_height = inner_area.height / paragraphs.len() as u16;

                for (i, paragraph) in paragraphs.iter().enumerate() {
                    let y_offset = i as u16 * paragraph_height;
                    let para_area = Rect {
                        x: inner_area.x,
                        y: inner_area.y + y_offset,
                        width: inner_area.width,
                        height: paragraph_height,
                    };

                    let default_style = Style::default().fg(self.theme.primary);
                    let style = paragraph.style.unwrap_or(default_style);

                    let widget = Paragraph::new(paragraph.text.clone())
                        .style(style)
                        .alignment(paragraph.alignment);

                    f.render_widget(widget, para_area);
                }
            }

            BoxContent::Custom { render_fn } => {
                render_fn(f, inner_area, &self.theme);
            }

            BoxContent::Loading { message } => {
                let loading_widget = Paragraph::new(message)
                    .style(Style::default().fg(self.theme.info))
                    .alignment(Alignment::Center);
                f.render_widget(loading_widget, inner_area);
            }

            BoxContent::Error { message } => {
                let error_widget = Paragraph::new(message)
                    .style(Style::default().fg(self.theme.error))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(error_widget, inner_area);
            }

            BoxContent::Empty {
                message,
                suggestion,
            } => {
                let full_message = if let Some(suggestion) = suggestion {
                    format!("{}\n\n{}", message, suggestion)
                } else {
                    message
                };

                let empty_widget = Paragraph::new(full_message)
                    .style(Style::default().fg(self.theme.muted))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(empty_widget, inner_area);
            }
        }
    }
}

/// Convenience functions for common box types
impl UniversalBox {
    /// Create a header box
    pub fn header<S: Into<String>>(title: S, theme: UnifiedTheme) -> Self {
        Self::new(theme.clone())
            .title(title)
            .text_styled("AWS CloudWatch TUI", Style::default().fg(theme.primary))
    }

    /// Create an info box
    pub fn info<S: Into<String>, T: Into<String>>(
        title: S,
        message: T,
        theme: UnifiedTheme,
    ) -> Self {
        Self::new(theme.clone())
            .title(title)
            .text_styled(message, Style::default().fg(theme.info))
    }

    /// Create a success box
    pub fn success<S: Into<String>, T: Into<String>>(
        title: S,
        message: T,
        theme: UnifiedTheme,
    ) -> Self {
        Self::new(theme.clone())
            .title(title)
            .text_styled(message, Style::default().fg(theme.success))
    }

    /// Create a warning box
    pub fn warning<S: Into<String>, T: Into<String>>(
        title: S,
        message: T,
        theme: UnifiedTheme,
    ) -> Self {
        Self::new(theme.clone())
            .title(title)
            .text_styled(message, Style::default().fg(theme.warning))
    }

    /// Create an error box
    pub fn error_box<S: Into<String>, T: Into<String>>(
        title: S,
        message: T,
        theme: UnifiedTheme,
    ) -> Self {
        Self::new(theme.clone()).title(title).error(message)
    }

    /// Create a loading box
    pub fn loading_box<S: Into<String>, T: Into<String>>(
        title: S,
        message: T,
        theme: UnifiedTheme,
    ) -> Self {
        Self::new(theme.clone()).title(title).loading(message)
    }
}

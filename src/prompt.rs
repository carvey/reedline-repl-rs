use reedline::{DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch};
use std::borrow::Cow;

#[derive(Clone)]
pub struct ReplPrompt {
    default: DefaultPrompt,
    prefix: String,
    indicator: String,
    timestamp: bool
}

impl Prompt for ReplPrompt {
    /// Use prefix as render prompt
    fn render_prompt_left(&self) -> Cow<str> {
        {
            Cow::Borrowed(&self.prefix)
        }
    }

    // if timestamp is true, render timestamp on right side of prompt. Otherwise render nothing on right side
    fn render_prompt_right(&self) -> Cow<str> {
        if self.timestamp {
            self.default.render_prompt_right()
        }
        else {
            Cow::Borrowed("")
        }
    }

    // if indicator is empty, use default indicator from reedline. Otherwise use indicator provided
    fn render_prompt_indicator(&self, edit_mode: PromptEditMode) -> Cow<str> {
        if self.indicator.is_empty() {
            self.default.render_prompt_indicator(edit_mode)
        }
        else {
            Cow::Borrowed(&self.indicator)
        }
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        self.default.render_prompt_multiline_indicator()
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        self.default
            .render_prompt_history_search_indicator(history_search)
    }
}

impl Default for ReplPrompt {
    fn default() -> Self {
        ReplPrompt::new("repl")
    }
}

impl ReplPrompt {
    /// Constructor for the default prompt, which takes the amount of spaces required between the left and right-hand sides of the prompt
    pub fn new(left_prompt: &str) -> ReplPrompt {
        ReplPrompt {
            prefix: left_prompt.to_string(),
            default: DefaultPrompt::default(),
            indicator: String::new(),
            timestamp: true
        }
    }

    pub fn update_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }

    pub fn update_indicator(&mut self, indicator: &str) {
        self.indicator = indicator.to_string();
    }

    pub fn show_timestamp(&mut self, hide: bool) {
        self.timestamp = hide;
    }
}

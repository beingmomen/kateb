pub struct VoiceCommandProcessor {
    enabled: bool,
    language: String,
}

pub struct ProcessResult {
    pub text: String,
    pub had_commands: bool,
    #[allow(dead_code)]
    pub delete_count: usize,
}

struct VoiceCommand {
    triggers: Vec<&'static str>,
    action: CommandAction,
}

enum CommandAction {
    Insert(&'static str),
    InsertLocalized { ar: &'static str, en: &'static str },
    DeleteLastWord,
}

impl VoiceCommandProcessor {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            language: "ar".to_string(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }

    pub fn process_text(&self, text: &str, language: &str) -> ProcessResult {
        if !self.enabled || text.trim().is_empty() {
            return ProcessResult {
                text: text.to_string(),
                had_commands: false,
                delete_count: 0,
            };
        }

        let commands = Self::get_commands();
        let mut result = text.to_string();
        let mut had_commands = false;
        let mut delete_count: usize = 0;

        for cmd in &commands {
            for trigger in &cmd.triggers {
                let replaced = Self::replace_standalone(&result, trigger, &cmd.action, language);
                if replaced.0 != result {
                    had_commands = true;
                    delete_count += replaced.1;
                    result = replaced.0;
                }
            }
        }

        result = Self::clean_extra_spaces(&result);

        ProcessResult {
            text: result,
            had_commands,
            delete_count,
        }
    }

    fn get_commands() -> Vec<VoiceCommand> {
        vec![
            VoiceCommand {
                triggers: vec!["سطر جديد", "new line", "newline"],
                action: CommandAction::Insert("\n"),
            },
            VoiceCommand {
                triggers: vec!["فقرة جديدة", "new paragraph"],
                action: CommandAction::Insert("\n\n"),
            },
            VoiceCommand {
                triggers: vec!["نقطة", "period", "dot", "full stop"],
                action: CommandAction::InsertLocalized { ar: ".", en: "." },
            },
            VoiceCommand {
                triggers: vec!["فاصلة", "comma"],
                action: CommandAction::InsertLocalized {
                    ar: "،",
                    en: ",",
                },
            },
            VoiceCommand {
                triggers: vec!["علامة استفهام", "question mark"],
                action: CommandAction::InsertLocalized {
                    ar: "؟",
                    en: "?",
                },
            },
            VoiceCommand {
                triggers: vec!["علامة تعجب", "exclamation mark", "exclamation point"],
                action: CommandAction::Insert("!"),
            },
            VoiceCommand {
                triggers: vec!["مسافة", "space"],
                action: CommandAction::Insert(" "),
            },
            VoiceCommand {
                triggers: vec!["احذف", "تراجع", "delete", "undo"],
                action: CommandAction::DeleteLastWord,
            },
        ]
    }

    fn is_word_boundary(c: char) -> bool {
        c.is_whitespace()
            || matches!(
                c,
                '.' | ',' | '?' | '!' | ';' | ':' | '،' | '؟' | '؛' | '"' | '\'' | '(' | ')'
                    | '-'
            )
    }

    fn replace_standalone(
        text: &str,
        trigger: &str,
        action: &CommandAction,
        language: &str,
    ) -> (String, usize) {
        let lower_text = text.to_lowercase();
        let lower_trigger = trigger.to_lowercase();
        let mut result = String::new();
        let mut delete_count: usize = 0;
        let mut last_end = 0;

        let trigger_len = lower_trigger.len();

        let mut search_start = 0;
        while let Some(pos) = lower_text[search_start..].find(&lower_trigger) {
            let abs_pos = search_start + pos;
            let end_pos = abs_pos + trigger_len;

            let before_ok = abs_pos == 0
                || lower_text[..abs_pos]
                    .chars()
                    .last()
                    .map_or(true, |c| Self::is_word_boundary(c));

            let after_ok = end_pos >= lower_text.len()
                || lower_text[end_pos..]
                    .chars()
                    .next()
                    .map_or(true, |c| Self::is_word_boundary(c));

            if before_ok && after_ok {
                result.push_str(&text[last_end..abs_pos]);

                match action {
                    CommandAction::Insert(s) => {
                        result.push_str(s);
                    }
                    CommandAction::InsertLocalized { ar, en } => {
                        if language == "ar" {
                            result.push_str(ar);
                        } else {
                            result.push_str(en);
                        }
                    }
                    CommandAction::DeleteLastWord => {
                        let trimmed = result.trim_end();
                        if let Some(last_space) = trimmed.rfind(|c: char| c.is_whitespace()) {
                            result = trimmed[..last_space].to_string();
                        } else {
                            result.clear();
                        }
                        delete_count += 1;
                    }
                }

                last_end = end_pos;
                search_start = end_pos;
            } else {
                let next_char_len = lower_text[abs_pos..]
                    .chars()
                    .next()
                    .map_or(1, |c| c.len_utf8());
                search_start = abs_pos + next_char_len;
            }
        }

        result.push_str(&text[last_end..]);
        (result, delete_count)
    }

    fn clean_extra_spaces(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_space = false;
        for ch in text.chars() {
            if ch == ' ' {
                if !prev_space {
                    result.push(ch);
                }
                prev_space = true;
            } else {
                prev_space = false;
                result.push(ch);
            }
        }
        result
    }
}

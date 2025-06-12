use std::{
    collections::{HashMap, HashSet},
    error::Error,
    process::Command,
};

use rustyline::{
    CompletionType, Config, EditMode, Editor, Helper,
    completion::Completer,
    highlight::Highlighter,
    hint::{Hinter, HistoryHinter},
    history::FileHistory,
    validate::Validator,
};

use crate::compiler::{
    lexer::Lexer,
    parser::{Parser, expression::Literal, syntax::Syntax},
};

struct ReplHelper {
    _hinter: HistoryHinter,
    completions: HashSet<String>,
}

impl Completer for ReplHelper {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if line.is_empty() {
            Ok((0, self.completions.iter().cloned().collect::<Vec<_>>()))
        } else {
            let mut head_index = 0;
            let line_as_bytes = line.as_bytes();

            for index in pos.saturating_sub(1)..0 {
                if line_as_bytes.iter().nth(index).is_some_and(|b| *b == b' ') {
                    head_index = index;
                    break;
                }
            }

            let uncompleted_len = line[head_index..].len().saturating_sub(head_index);

            let completions = self
                .completions
                .iter()
                .cloned()
                .filter(|c| c.starts_with(&line[head_index..]))
                .map(|mut c| {
                    for _ in 0..uncompleted_len {
                        c.remove(0);
                    }

                    c
                })
                .collect::<Vec<_>>();

            Ok((pos, completions))
        }
    }
}

impl Highlighter for ReplHelper {}
impl Validator for ReplHelper {}

impl Hinter for ReplHelper {
    type Hint = String;
}

impl Helper for ReplHelper {}

pub struct Repl {
    editor: Editor<ReplHelper, FileHistory>,
    variables: HashMap<String, Literal>,
}

impl Repl {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Vi)
            .build();

        let helper = ReplHelper {
            _hinter: HistoryHinter::new(),
            completions: HashSet::from([
                "exit".to_string(),
                "let".to_string(),
                "clear".to_string(),
                "list".to_string(),
                "help".to_string(),
            ]),
        };

        let mut editor = Editor::with_config(config)?;

        editor.set_helper(Some(helper));

        Ok(Self {
            editor,
            variables: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "Welcome to scr (simple calculation repl), double press tab to show available commands in the completion list.\n"
        );

        loop {
            let line = self.editor.readline("scr > ")?;
            self.editor.add_history_entry(&line)?;

            let mut lexer = Lexer::new(line);
            let tokens = lexer.tokenize();
            let mut parser = Parser::new(tokens);
            let syntax = parser.parse();

            match syntax {
                Syntax::Command(name) => match name.as_str() {
                    "exit" => break Ok(()),

                    "clear" => {
                        Command::new("clear").spawn()?.wait()?;
                    }

                    "list" => {
                        for (name, value) in self.variables.iter() {
                            println!("{name} = {value}");
                        }
                    }

                    "help" => println!(
                        "
scr (simple calculation REPL) is a simplistic math REPL for quick calculations:
    Available commands:
        exit    - Exits the REPL.
        clear   - Clears the current terminal screen.
        let     - Defines variables.
        list    - Lists all variables in order with their values respectfully.
        help    - Prints this message.
"
                    ),

                    _ => {}
                },

                Syntax::Expression(expression) => {
                    println!("{}", expression.evaluate(&self.variables));
                }

                Syntax::Variable(name, value) => {
                    self.editor
                        .helper_mut()
                        .unwrap()
                        .completions
                        .insert(name.clone());
                    self.variables.insert(name, value.evaluate(&self.variables));
                }

                Syntax::Nop => {}
            }
        }
    }
}

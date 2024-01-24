use imessage_database::tables::messages::{Message, BubbleType};
use regex::Regex;

/// Make necessary replacements so that the text is ready for insertion
/// into latex
fn latex_escape(text: String) -> String {
    // TODO: gotta be a more efficient way to do this
    let escaped = text 
        // first, a bunch of weird characters replaced with ascii
        .replace("’", "'")
        .replace("“", "\"")
        .replace("”", "\"")
        .replace("…", "...")
        // now, actual latex escapes
        .replace(r"\", r"\textbackslash\ ")
        .replace("$", r"\$")
        .replace("%", r"\%")
        .replace("&", r"\&")
        .replace("_", r"\_")
        .replace("^", r"\textasciicircum\ ")
        .replace("~", r"\textasciitilde\ ")
        .replace("#", r"\#")
        .replace(r"{", r"\{")
        .replace(r"}", r"\}")
        // this last one is the "variation selector" which I think determines whether an emoji
        // should be displayed big or inline. The latex font doesn't support it, so we just strip it out.
        // More info here: https://stackoverflow.com/questions/38100329/what-does-u-ufe0f-in-an-emoji-mean-is-it-the-same-if-i-delete-it
        .replace("\u{FE0F}", "");

    // Now, we wrap emojis in {\emojifont XX}. The latex template has a different font for emojis, and
    // this allows emojis to use that font
    // TODO: Somehow move this regex out so we only compile it once
    let emoji_regex = Regex::new(r"(\p{Extended_Pictographic}+)").expect("Couldn't compile demoji regex");
    let demojid = emoji_regex.replace_all(&escaped, "{\\emojifont $1}").into_owned();

    demojid

}

struct LatexMessage {
    is_from_me: bool,
    body_text: Option<String>,
    attachment_count: i32
}

impl LatexMessage {
    fn render(self) -> String {
        let content = match self.body_text {
            Some(ref text) => latex_escape(text.to_string()), // probably not ideal to be cloning here
            None => "< EMPTY MESSAGE >".to_string(),
        };

        let to_write = match self.is_from_me {
            // god generating latex code is so annoying with the escapes
            true => format!("\\leftmsg{{{}}}\n\n", content),
            false => format!("\\rightmsg{{{}}}\n\n", content),
        };

        to_write
    }
}

pub fn render_message(msg: &Message) -> String {
    let parts = msg.body();

    let mut latex_msg = LatexMessage { is_from_me: msg.is_from_me, body_text: None, attachment_count: 0 };

    for part in parts {
        match part {
            BubbleType::Text(text) => { latex_msg.body_text = Some(text.to_owned())},
            BubbleType::Attachment => { latex_msg.attachment_count += 1 }
            _ => ()
        }
    }

    latex_msg.render()
}
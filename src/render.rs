use imessage_database::tables::messages::Message;
use regex::Regex;

/// Make necessary replacements so that the text is ready for insertion
/// into latex
fn latex_escape(text: String) -> String {
    // TODO: gotta be a more efficient way to do this
    let escaped = text 
        .replace("’", "'")
        .replace("“", "\"")
        .replace("”", "\"")
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
        .replace("\u{FFFC}", "[OBJ]");
        // .replace("\u{FFFD}", "[OBJ]");

    // Now, we wrap emojis in {\emojifont XX}. The latex template has a different font for emojis, and
    // this allows emojis to use that font
    // TODO: Somehow move this regex out so we only compile it once
    let emoji_regex = Regex::new(r"(\p{Extended_Pictographic}+)").expect("Couldn't compile demoji regex");
    let demojid = emoji_regex.replace_all(&escaped, "{\\emojifont $1}").into_owned();

    demojid

}

pub fn render_message(msg: &Message) -> String {
    let content = match msg.text {
        Some(ref text) => latex_escape(text.to_string()), // probably not ideal to be cloning here
        None => "< EMPTY MESSAGE >".to_string(),
    };

    let to_write = match msg.is_from_me {
        // god generating latex code is so annoying with the escapes
        true => format!("\\leftmsg{{{}}}\n\n", content),
        false => format!("\\rightmsg{{{}}}\n\n", content),
    };

    to_write
}
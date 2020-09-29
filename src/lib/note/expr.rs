pub type Span = (usize, usize);

pub struct Note {
    pub paragraphs: Vec<Paragraph>,
}

pub struct Paragraph {
    pub parts: Vec<TextElement>,
}

pub enum TextElement {
    Prose(Span),
    Link { title: Span, href: Span },
    Mono(Span),
}

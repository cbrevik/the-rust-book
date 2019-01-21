struct Context<'s>(&'s str);

// Basically, s-lifetime subtypes c
// In other words, s will live "at least"
// as long as c, (or outlive)
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// The T: 'a syntax specifies that T can be any type, but if it contains any references, the references must live at least as long as 'a.
struct Ref<'a, T: 'a>(&'a T);

fn main() {
    let context = Context("Hello, world!");

    let result = Parser { context: &context }.parse();

    match result {
        Err(s) => println!("{}", s),
        _ => println!("Should never happen"),
    }

    let result = parse_context(context);

    match result {
        Err(s) => println!("{} Again!", s),
        _ => println!("Should never happen"),
    }

    // To be honest this chapter barely made sense to me.
}

struct StrWrap<'a>(&'a str);

// verbose
//impl<'a> fmt::Debug for StrWrap<'a> {}

// elided, with anon-lifetime _
//impl fmt::Debug for StrWrap<'_> {}

enum Func {
    Type,
    Params,
    Body,
}

enum Body {
    Stat,
    Cond,
    Loop,
}

enum Stat {
    Assign,
    Return,
}

enum Assign {
    ID,
    Exp,
}

enum Exp {
    Val,
    Operator,
}

enum Operator {
    Plus,
    Minus,
}

enum Value {
    Number,
    Str,
    Char,
}

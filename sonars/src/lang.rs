enum MExpr{
    Sin(E),
    Add(E, E),
    Mul(E, E),
    Div(E, E),
    Sub(E, E),
    Mod(E, E),
    Pow(E,E),
    Num(f32),
    Invoke(String),
    Time
}

enum MStmt{
    Assign(String, Vec<String>, MExpr)
}


type E = Box<MExpr>;

pub type Index = u32;

#[derive(Debug, Clone)]
pub struct Env(Vec<Value>);

impl Env {
    /// Create an empty environment.
    pub fn empty() -> Self {
        Self(Vec::with_capacity(100))
    }

    /// Dereference a variable, getting the bound value.
    pub fn get(&self, idx: Index) -> Option<&Value> {
        self.0.get(self.0.len().wrapping_sub(idx as usize + 1))
    }

    pub fn extend(&mut self, val: Value) {
        self.0.push(val)
    }
}

#[derive(Debug)]
pub enum Term {
    Var(Index),
    Lam(TermRef),
    App(TermRef, TermRef),
}

#[derive(Debug, Clone, Copy)]
pub struct TermRef(u32);

#[derive(Debug, Clone)]
pub enum Value {
    Closure(TermRef, Env),
}

#[derive(Debug)]
pub struct TermArena(Vec<Term>);

impl TermArena {
    /// Create an empty arena.
    pub fn default() -> Self {
        Self(Vec::with_capacity(1000))
    }

    /// Dereference an AST node reference, obtaining the underlying `Term`.
    pub fn get(&self, term: TermRef) -> &Term {
        &self.0[term.0 as usize]
    }

    /// Add an termession to the arena and get a reference to it.
    pub fn allocate(&mut self, term: Term) -> TermRef {
        let idx = self.0.len();
        self.0.push(term);
        TermRef(idx.try_into().expect("too many terms in the arena"))
    }

    pub fn var(&mut self, idx: Index) -> TermRef {
        self.allocate(Term::Var(idx))
    }

    pub fn lam(&mut self, body: TermRef) -> TermRef {
        self.allocate(Term::Lam(body))
    }

    pub fn app(&mut self, fn_term: TermRef, arg_term: TermRef) -> TermRef {
        self.allocate(Term::App(fn_term, arg_term))
    }
}

pub fn eval(arena: &TermArena, term: TermRef, env: &Env) -> Result<Value, String> {
    match arena.get(term) {
        Term::Var(index) => env
            .get(*index)
            .ok_or(format!("variable not found: {}", index))
            .cloned(), // FIXME, don't clone
        Term::Lam(_body) => Ok(Value::Closure(term, env.clone())),
        Term::App(fn_term, arg_term) => {
            let Value::Closure(fn_val, defenv) = eval(arena, *fn_term, env)?;
            let arg_val = eval(arena, *arg_term, env)?; // TODO do we want to be eager?
            match arena.get(fn_val) {
                Term::Lam(body) => {
                    let mut new_env = defenv;
                    new_env.extend(arg_val);
                    eval(arena, *body, &new_env)
                }
                _ => Err("function value must evaluate to a lambda term".to_string()),
            }
        }
    }
}

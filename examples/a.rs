#![allow(unused)]
#![allow(dead_code)]

mod ast {
    #[derive(Clone)]
    pub enum Ty {
        ImplTrait(TraitRef),
        NamedType(String, Vec<Ty>),
    }

    #[derive(Clone)]
    pub struct TraitRef {
        pub trait_name: String,
        pub parameters: Parameters,
    }

    #[derive(Clone)]
    pub enum Parameter {
        Ty(Ty),
        Lifetime(Lifetime),
    }

    #[derive(Clone)]
    pub struct Lifetime {
        name: String,
    }

    #[derive(Clone)]
    pub enum Parameters {
        AngleBracket(Vec<Parameter>),
        Parenthesized(Vec<Ty>),
    }

    #[derive(Clone)]
    pub struct Signature {
        pub inputs: Vec<Ty>,
        pub output: Ty,
    }
}

struct Context {}

impl Context {
    fn lower_signature(&mut self, sig: &ast::Signature) {
        for input in &sig.inputs {
            self.lower_ty(input);
        }

        self.lower_ty(&sig.output);
    }

    fn lower_trait_ref(&mut self, trait_ref: &ast::TraitRef) {
        match &trait_ref.parameters {
            ast::Parameters::AngleBracket(parameters) => {
                self.lower_angle_bracket_parameters(&parameters)
            }
            ast::Parameters::Parenthesized(types) => {
                let parameters: Vec<_> = types.iter().cloned().map(ast::Parameter::Ty).collect();
                self.lower_angle_bracket_parameters(&parameters)
            }
        }
    }

    fn lower_angle_bracket_parameters(&mut self, parameters: &[ast::Parameter]) {
        for p in parameters {
            match p {
                ast::Parameter::Ty(ty) => self.lower_ty(ty),
                ast::Parameter::Lifetime(lt) => self.lower_lifetime(lt),
            }
        }
    }

    fn lower_ty(&mut self, ty: &ast::Ty) {
        match ty {
            // ... lots of stuff here
            // A type like `impl Trait`
            ast::Ty::ImplTrait(trait_ref) => {
                do_something_with(trait_ref);
            }

            // A type like `Vec<T>`, where `Vec` is the name and
            // `[T]` are the `parameters`
            ast::Ty::NamedType(name, parameters) => {
                for parameter in parameters {
                    self.lower_ty(parameter);
                }
            }
        }
    }

    fn lower_lifetime(&mut self, lt: &ast::Lifetime) {
        do_something_with(lt);
    }
}

fn do_something_with<T>(t: T) {}

fn main() {}

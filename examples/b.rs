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

mod hir {
    // just placeholders

    pub struct Ty;

    pub struct TraitRef;

    pub struct Signature;

    pub struct Parameters;

    pub struct Lifetime;
}
struct Context<'ast> {
    impl_trait_tys: Vec<&'ast ast::TraitRef>,
}

impl<'ast> Context<'ast> {
    fn new() -> Self {
        Self {
            impl_trait_tys: Vec::new(),
        }
    }

    fn lower_signature(&mut self, sig: &'ast ast::Signature) -> hir::Signature {
        // mark how many impl-traits we've seen so far
        let len = self.impl_trait_tys.len();

        // process input types
        for input in &sig.inputs {
            self.lower_ty(input);
        }

        // process any impl traits that we saw in those input types,
        // removing them from the vector
        for trait_ref in self.impl_trait_tys.drain(len..) {
            do_something_with(trait_ref);
        }

        self.lower_ty(&sig.output);

        hir::Signature
    }

    fn lower_trait_ref(&mut self, trait_ref: &'ast ast::TraitRef) -> hir::TraitRef {
        match &trait_ref.parameters {
            ast::Parameters::AngleBracket(parameters) => {
                self.lower_angle_bracket_parameters(&parameters);
            }
            ast::Parameters::Parenthesized(types) => {
                let parameters: Vec<_> = types.iter().cloned().map(ast::Parameter::Ty).collect();
                self.lower_angle_bracket_parameters(&parameters);
            }
        }

        hir::TraitRef
    }

    fn lower_angle_bracket_parameters(
        &mut self,
        parameters: &'ast [ast::Parameter],
    ) -> hir::Parameters {
        for p in parameters {
            match p {
                ast::Parameter::Ty(ty) => {
                    let hir_ty = self.lower_ty(ty);
                    // ...
                }
                ast::Parameter::Lifetime(lt) => {
                    let hir_lifetime = self.lower_lifetime(lt);
                    // ...
                }
            }
        }

        hir::Parameters
    }

    fn lower_ty(&mut self, ty: &'ast ast::Ty) -> hir::Ty {
        match ty {
            // ... lots of stuff here
            // A type like `impl Trait`
            ast::Ty::ImplTrait(trait_ref) => {
                do_something_with(trait_ref);
                self.impl_trait_tys.push(trait_ref);
            }

            // A type like `Vec<T>`, where `Vec` is the name and
            // `[T]` are the `parameters`
            ast::Ty::NamedType(name, parameters) => {
                for parameter in parameters {
                    self.lower_ty(parameter);
                }
            }
        }

        hir::Ty
    }

    fn lower_lifetime(&mut self, lt: &'ast ast::Lifetime) -> hir::Lifetime {
        do_something_with(lt);

        hir::Lifetime
    }
}

fn do_something_with<T>(t: T) {}

fn main() {
    let context = Context::new();
}

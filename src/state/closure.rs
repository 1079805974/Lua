use crate::binary::chunk::Prototype;
use crate::api::RustFn;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::state::lua_value::LuaValue;

pub struct Closure {
    pub proto: Rc<Prototype>,//lua closure
    pub rust_fn: Option<RustFn>,//rust closure
    pub upvalues:Vec<Rc<LuaValue>>,
    rdm: usize,
}

//TODO::?usage?
impl Hash for Closure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}

impl Closure {
    pub fn new_fake_closure() -> Closure {
        Closure {
            proto: new_empty_prototype(), // TODO
            rust_fn: None,
            rdm: super::math::random(),
            upvalues: Vec::new()
        }
    }

    pub fn new_lua_closure(proto: Rc<Prototype>) -> Closure {
        Closure {
            upvalues: Vec::with_capacity(proto.upvalues.len()),
            proto,
            rust_fn: None,
            rdm: super::math::random()
        }
    }

    pub fn new_rust_closure(f: RustFn,n_upvals: usize) -> Closure {
        Closure {
            proto: new_empty_prototype(), // TODO
            rust_fn: Some(f),
            rdm: super::math::random(),
            upvalues:Vec::with_capacity(n_upvals)
        }
    }
}

fn new_empty_prototype() -> Rc<Prototype> {
    Rc::new(Prototype {
        source: None, // debug
        line_defined: 0,
        last_line_defined: 0,
        num_params: 0,
        is_vararg: 0,
        max_stack_size: 0,
        code: vec![],
        constants: vec![],
        upvalues: vec![],
        protos: vec![],
        line_info: vec![],     // debug
        loc_vars: vec![],      // debug
        upvalue_names: vec![], // debug
    })
}

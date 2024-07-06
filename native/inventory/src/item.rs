use rustler::Encoder;
use rustler::Env;
use rustler::Term;

#[derive(Clone, Debug)]
pub struct Item {
    pub token: String,
    pub type_id: u32,
    pub cfg_id: u64,
    pub amount: u64,
}

impl Item {
    pub fn new(token: String, type_id: u32, cfg_id: u64, amount: u64) -> Item {
        Item {
            token,
            type_id,
            cfg_id,
            amount,
        }
    }
}

impl Encoder for Item {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (
            self.token.encode(env),
            self.type_id,
            self.cfg_id,
            self.amount,
        )
            .encode(env)
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum OpType {
    Incr = 1,
    Decr = 2,
    New = 3,
    Delete = 4,
}

pub struct Op {
    pub op_type: OpType,
    pub token: String,
    pub type_id: u32,
    pub cfg_id: u64,
    pub amount: u64,
}
impl Encoder for OpType {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            OpType::Incr => 1.encode(env),
            OpType::Decr => 2.encode(env),
            OpType::New => 3.encode(env),
            OpType::Delete => 4.encode(env),
        }
    }
}

impl Op {
    pub fn new(op_type: OpType, token: String, type_id: u32, cfg_id: u64, amount: u64) -> Op {
        Op {
            op_type,
            token,
            type_id,
            cfg_id,
            amount,
        }
    }
}
impl Encoder for Op {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (
            self.token.encode(env),
            self.op_type.encode(env),
            self.amount,
        )
            .encode(env)
    }
}

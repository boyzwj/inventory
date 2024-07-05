use rustler::Encoder;
use rustler::Env;
use rustler::Term;


#[derive(Clone, Debug)]
pub struct Item {
   pub token: String,
   pub type_id: u32,
   pub cfg_id: u64,
   pub amount: u64
}

impl Item {
    pub fn new(token: String, type_id: u32, cfg_id: u64, amount: u64) -> Item {
        Item { token, type_id, cfg_id, amount }
    }
}

impl Encoder for Item {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (self.token.encode(env),self.type_id,self.cfg_id,self.amount).encode(env)
    }
}

pub struct Op{
    pub token: String,
    pub op_type: u32,
    pub amount: u64
}

impl Encoder for Op{
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (self.token.encode(env),self.op_type,self.amount).encode(env)
    }
}
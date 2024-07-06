use dashmap::DashMap;
use rustler::resource::ResourceArc;
use rustler::types::tuple::get_tuple;
use rustler::{Atom, Env, Term};
use std::collections::HashSet;
mod item;
use crate::item::Item;
use crate::item::Op;
use crate::item::OpType;
mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,

        // Resource Atoms
        bad_reference,
        lock_fail,

        // Success Atoms
        added,
        duplicate,
        removed,

        // Error Atoms
        unsupported_type,
        not_found,
        nil,
        illegal_ops
    }
}

pub struct BagResource {
    bag: Bag,
}

type BagArc = ResourceArc<BagResource>;

rustler::init!(
    "Elixir.Inventory.Native",
    [
        new,
        add,
        get,
        get_by_type,
        get_by_cfg_id,
        amount,
        amount_by_type,
        amount_by_cfg_id,
        to_list,
        verify_ops,
        do_ops
    ],
    load = load
);

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(BagResource, env);
    true
}

pub struct Bag {
    items: DashMap<String, Item>,
    type_indices: DashMap<u32, HashSet<String>>,
    cfg_indices: DashMap<u64, HashSet<String>>,
}

impl Bag {
    pub fn new() -> Bag {
        Bag {
            items: DashMap::new(),
            type_indices: DashMap::new(),
            cfg_indices: DashMap::new(),
        }
    }

    pub fn incr(&self, item: Item) {
        if self.items.contains_key(&item.token) {
            let mut oitem = self.items.get_mut(&item.token).unwrap();
            oitem.amount += item.amount;
        }
    }

    pub fn decr(&self, item: Item) {
        if self.items.contains_key(&item.token) {
            let mut oitem = self.items.get_mut(&item.token).unwrap();
            oitem.amount -= item.amount;
            if oitem.amount == 0 {
                self.type_indices
                    .get_mut(&oitem.type_id)
                    .map(|mut set| set.remove(&oitem.token));
                self.cfg_indices
                    .get_mut(&oitem.cfg_id)
                    .map(|mut set| set.remove(&oitem.token));
                self.items.remove(&oitem.token);
            }
        }
    }

    pub fn add(&self, item: Item) {
        if self.items.contains_key(&item.token) {
            self.items.get_mut(&item.token).unwrap().amount += item.amount;
        } else {
            self.items.insert(item.token.clone(), item.clone());
            self.type_indices
                .entry(item.type_id)
                .or_default()
                .insert(item.token.clone());
            self.cfg_indices
                .entry(item.cfg_id)
                .or_default()
                .insert(item.token.clone());
        }
    }

    pub fn get_by_type(&self, type_id: u32) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        if self.type_indices.contains_key(&type_id) {
            let tokens = self.type_indices.get(&type_id).unwrap();
            for token in tokens.iter() {
                let item = self.items.get(token).unwrap().clone();
                result.push(item);
            }
        }
        result
    }

    pub fn get_by_cfg_id(&self, cfg_id: u64) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        if self.cfg_indices.contains_key(&cfg_id) {
            let tokens = self.cfg_indices.get(&cfg_id).unwrap();
            for token in tokens.iter() {
                let item = self.items.get(token).unwrap().clone();
                result.push(item);
            }
        }
        result
    }

    pub fn amount_by_cfg_id(&self, cfg_id: u64) -> u64 {
        let mut result: u64 = 0;
        if self.cfg_indices.contains_key(&cfg_id) {
            let tokens = self.cfg_indices.get(&cfg_id).unwrap();
            for token in tokens.iter() {
                let item = self.items.get(token).unwrap();
                result += item.amount;
            }
        }
        result
    }

    pub fn amount_by_type(&self, type_id: u32) -> u64 {
        let mut result: u64 = 0;
        if self.type_indices.contains_key(&type_id) {
            let tokens = self.type_indices.get(&type_id).unwrap();
            for token in tokens.iter() {
                let item = self.items.get(token).unwrap();
                result += item.amount;
            }
        }
        result
    }

    pub fn to_list(&self) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        for item in self.items.iter() {
            result.push(item.clone());
        }
        result
    }
    //ops type 0: cost, 1: add
    pub fn verify_ops(&self, ops: &Vec<Op>) -> bool {
        for op in ops {
            if op.op_type == OpType::Decr {
                match self.items.get(&op.token) {
                    Some(item) => {
                        if item.amount < op.amount {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }
        return true;
    }

    pub fn do_ops(&self, ops: &[Op]) -> Vec<(OpType, Item)> {
        let mut effect_items = Vec::new();
        for op in ops {
            match op.op_type {
                OpType::Decr => {
                    if let Some(mut item) = self.items.get_mut(&op.token) {
                        item.amount -= op.amount;
                        if item.amount == 0 {
                            self.type_indices
                                .get_mut(&item.type_id)
                                .map(|mut set| set.remove(&item.token));
                            self.cfg_indices
                                .get_mut(&item.cfg_id)
                                .map(|mut set| set.remove(&item.token));
                            effect_items.push((OpType::Delete, item.clone()));
                        } else {
                            effect_items.push((OpType::Decr, item.clone()));
                        }
                    }
                }
                OpType::Incr => {
                    if self.items.contains_key(&op.token) {
                        let mut item = self.items.get_mut(&op.token).unwrap();
                        item.amount += op.amount;
                        effect_items.push((OpType::Incr, item.clone()));
                    } else {
                        let new_item =
                            Item::new(op.token.clone(), op.type_id, op.cfg_id, op.amount);
                        self.type_indices
                            .entry(op.type_id)
                            .or_default()
                            .insert(op.token.clone());
                        self.cfg_indices
                            .entry(op.cfg_id)
                            .or_default()
                            .insert(op.token.clone());
                        self.items.insert(op.token.clone(), new_item.clone());
                        effect_items.push((OpType::New, new_item));
                    }
                }
                _ => {}
            }
        }
        effect_items
    }
}

#[rustler::nif]
fn new() -> (Atom, BagArc) {
    let resource = ResourceArc::new(BagResource { bag: Bag::new() });
    (atoms::ok(), resource)
}
#[rustler::nif]
fn add(resource: BagArc, token: String, type_id: u32, cfg_id: u64, num: u64) -> Atom {
    resource.bag.add(Item::new(token, type_id, cfg_id, num));
    atoms::ok()
}

#[rustler::nif]
fn get(resource: BagArc, token: String) -> Result<Item, Atom> {
    resource
        .bag
        .items
        .get(&token)
        .map_or(Err(atoms::not_found()), |item| Ok(item.clone()))
}

#[rustler::nif]
fn get_by_type(resource: BagArc, type_id: u32) -> Result<Vec<Item>, Atom> {
    Ok(resource.bag.get_by_type(type_id))
}

#[rustler::nif]
fn get_by_cfg_id(resource: BagArc, cfg_id: u64) -> Result<Vec<Item>, Atom> {
    Ok(resource.bag.get_by_cfg_id(cfg_id))
}

#[rustler::nif]
fn amount(resource: BagArc, token: String) -> u64 {
    resource.bag.items.get(&token).map_or(0, |item| item.amount)
}

#[rustler::nif]
fn amount_by_type(resource: BagArc, type_id: u32) -> u64 {
    resource.bag.amount_by_type(type_id)
}

#[rustler::nif]
fn amount_by_cfg_id(resource: BagArc, cfg_id: u64) -> u64 {
    resource.bag.amount_by_cfg_id(cfg_id)
}

#[rustler::nif]
fn to_list(resource: BagArc) -> Vec<Item> {
    resource.bag.to_list()
}

#[rustler::nif]
fn verify_ops(resource: BagArc, term: Term) -> bool {
    match convert_ops(&term) {
        Some(ops) => resource.bag.verify_ops(&ops),
        None => false,
    }
}

#[rustler::nif]
fn do_ops(resource: BagArc, term: Term) -> Result<Vec<(OpType, Item)>, Atom> {
    let ops = convert_ops(&term).ok_or(atoms::unsupported_type())?;
    if resource.bag.verify_ops(&ops) {
        Ok(resource.bag.do_ops(&ops))
    } else {
        Err(atoms::illegal_ops())
    }
}

fn convert_ops(term: &Term) -> Option<Vec<Op>> {
    if term.is_list() {
        match term.decode::<Vec<Term>>() {
            Ok(l) => {
                let mut ops = Vec::new();
                for item in l {
                    if let Ok(op) = convert_term_to_op(&item) {
                        ops.push(op);
                    } else {
                        return None;
                    }
                }
                Some(ops)
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

fn convert_term_to_op(term: &Term) -> Result<Op, Atom> {
    if term.is_tuple() {
        match get_tuple(*term) {
            Ok(t) => {
                if t.len() != 5 {
                    return Err(atoms::unsupported_type());
                }
                if !t[0].is_integer() {
                    return Err(atoms::unsupported_type());
                }
                if !t[1].is_binary() {
                    return Err(atoms::unsupported_type());
                }
                if !t[2].is_integer() {
                    return Err(atoms::unsupported_type());
                }
                if !t[3].is_integer() {
                    return Err(atoms::unsupported_type());
                }
                if !t[4].is_integer() {
                    return Err(atoms::unsupported_type());
                }
                let op_type: OpType = match t[0].decode().unwrap() {
                    1 => OpType::Incr,
                    2 => OpType::Decr,
                    3 => OpType::New,
                    4 => OpType::Delete,
                    _ => return Err(atoms::unsupported_type()),
                };
                let token: String = t[1].decode().unwrap();
                let type_id: u32 = t[2].decode().unwrap();
                let cfg_id: u64 = t[3].decode().unwrap();
                let num: u64 = t[4].decode().unwrap();
                Ok(Op::new(op_type, token, type_id, cfg_id, num))
            }
            Err(_) => Err(atoms::unsupported_type()),
        }
    } else {
        Err(atoms::unsupported_type())
    }
}

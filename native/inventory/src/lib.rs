use dashmap::DashMap;
use rustler::{Atom, Env, Term};
// use rustler::types::tuple::get_tuple;
use rustler::resource::ResourceArc;
use std::sync::RwLock;
mod item;
use crate::item::Item;

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
        max_size_exceeded,
    }
}


pub struct BagResource(RwLock<Bag>);
type BagArc = ResourceArc<BagResource>;

rustler::init!("Elixir.Inventory.Native", [
    new,
    add,
    get,
    get_by_type,
    get_by_cfg_id,
    enough,
    to_list
    ],
    load = load);

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(BagResource, env);
    true
}

pub struct Bag {
    items: DashMap<String, Item>,
    type_indexs: DashMap<u32, Vec<String>>,
    cfg_indexs: DashMap<u64, Vec<String>>,
}

impl Bag {
    pub fn new() -> Bag {
        Bag { 
            items: DashMap::new(),
            type_indexs: DashMap::new(),
            cfg_indexs: DashMap::new(),
        }
    }

    pub fn add(&mut self, item: Item) {
        if self.items.contains_key(&item.token) {
            let mut oitem = self.items.get_mut(&item.token).unwrap();
            oitem.amount += item.amount;
        }
        else
        {
            self.type_indexs.entry(item.type_id).or_insert(Vec::new()).push(item.token.clone());
            self.cfg_indexs.entry(item.cfg_id).or_insert(Vec::new()).push(item.token.clone());
            self.items.insert(item.token.clone(), item);
        }

    }
    pub fn get_by_type(&self, type_id: u32) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        if self.type_indexs.contains_key(&type_id) {
            let tokens = self.type_indexs.get(&type_id).unwrap();
            for token in tokens.to_vec() {
                let item = self.items.get(&token).unwrap().clone();
                result.push(item);
            }
        }
        result
    }

    pub fn get_by_cfg_id(&self, cfg_id: u64) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        if self.cfg_indexs.contains_key(&cfg_id) {
            let tokens = self.cfg_indexs.get(&cfg_id).unwrap();
            for token in tokens.to_vec() {
                let item = self.items.get(&token).unwrap().clone();
                result.push(item);
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


}



#[rustler::nif]
fn new() -> (Atom,BagArc) {
    let resource = ResourceArc::new(BagResource(RwLock::new(Bag::new())));
    (atoms::ok(), resource)
}


#[rustler::nif]
fn add(resource: ResourceArc<BagResource>,token: String, type_id: u32, cfg_id: u64, amount: u64) -> Atom {
    // TODO
    let mut bag = match resource.0.write() {
        Ok(bag) => bag,
        Err(_) => {
            return atoms::lock_fail();
        }
    };
    bag.add(Item::new(token, type_id, cfg_id, amount));
    atoms::ok()
}

#[rustler::nif]
fn get(resource: ResourceArc<BagResource>, token: String) -> Result<Item, Atom> {
    let bag = match resource.0.read() {
        Ok(bag) => bag,
        Err(_) => {
            return Err(atoms::lock_fail());
        }
    };
    if bag.items.contains_key(&token) {
        let item = bag.items.get(&token).unwrap();
        return Ok(item.clone());
    }
    else {
        return Err(atoms::nil());
    }
}

#[rustler::nif]
fn get_by_type(resource: ResourceArc<BagResource>, type_id: u32) -> Result<Vec<Item>,Atom> {
    let bag = match resource.0.read() {
        Ok(bag) => bag,
        Err(_) => {
            return Err(atoms::lock_fail());
        }
    };
    let result = bag.get_by_type(type_id);
    return Ok(result);
}

#[rustler::nif]
fn get_by_cfg_id(resource: ResourceArc<BagResource>, cfg_id: u64) -> Result<Vec<Item>,Atom> {
    let bag = match resource.0.read() {
        Ok(bag) => bag,
        Err(_) => {
            return Err(atoms::lock_fail());
        }
    };
    let result = bag.get_by_cfg_id(cfg_id);
    return Ok(result);
}

#[rustler::nif]
fn enough(resource: ResourceArc<BagResource>,cfg_id: u64, amount: u64) -> Result<bool,Atom> {
    let bag = match resource.0.read() {
        Ok(bag) => bag,
        Err(_) => {
            return Err(atoms::lock_fail());
        }
    };
    let items = bag.get_by_cfg_id(cfg_id);
    let mut total = 0;
    for item in items {
        total += item.amount;
    }
    return Ok(total >= amount);
}

#[rustler::nif]
fn to_list(resource: ResourceArc<BagResource>) -> Vec<Item> {
    let bag = match resource.0.read() {
        Ok(bag) => bag,
        Err(_) => {
            return Vec::new();
        }
    };

    bag.to_list()
}



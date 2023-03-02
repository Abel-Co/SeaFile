use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

use crate::module::daisy::{Daisy, dao};

pub async fn daisy_user(id: i64, level: isize) -> Option<Vec<Rc<RefCell<Daisy>>>> {
    let list = vec![Rc::new(RefCell::new(Daisy { id, level: Some(level), ..Default::default() }))];
    let mut p = list.to_owned();
    for _ in 0..5 {
        let ids: Vec<i64> = p.iter().map(|x| get_id(x)).collect();
        if ids.len() == 0 {
            break;
        }
        p = cycle_proxy(ids, &p).await;
    }
    list.first().unwrap().take().children
}

async fn cycle_proxy(ids: Vec<i64>, p: &Vec<Rc<RefCell<Daisy>>>) -> Vec<Rc<RefCell<Daisy>>> {
    let daisy = dao::get(&ids).await;
    let r_map = group_by_parent(&daisy);
    p.iter().for_each(|x| {
        if let Some(vec) = r_map.get(&get_id(x)) {
            let p_level = get_level(x);
            let children = vec.iter().map(|x| fix_level(x.clone(), p_level)).collect();
            x.replace(x.take().set_children(Some(children)));
        }
    });
    daisy
}

fn group_by_parent(daisy: &Vec<Rc<RefCell<Daisy>>>) -> HashMap<i64, Vec<Rc<RefCell<Daisy>>>> {
    let group_by = daisy.iter().group_by(|&x| get_parent_id(x));
    group_by.into_iter().map(|(k, v)| (k, v.cloned().collect())).into_iter().collect()
}

fn get_id(daisy: &Rc<RefCell<Daisy>>) -> i64 {
    let x = daisy.take();
    let id = x.id.clone();
    daisy.replace(x);
    id
}

fn get_parent_id(daisy: &Rc<RefCell<Daisy>>) -> i64 {
    let x = daisy.take();
    let id = x.parent.clone();
    daisy.replace(x);
    id
}

fn get_level(daisy: &Rc<RefCell<Daisy>>) -> isize {
    let x = daisy.take();
    let level = x.level.unwrap();
    daisy.replace(x);
    level
}

fn fix_level(daisy: Rc<RefCell<Daisy>>, level: isize) -> Rc<RefCell<Daisy>> {
    let mut x = daisy.take();
    x.level = Some(level + 1);
    daisy.replace(x);
    daisy
}